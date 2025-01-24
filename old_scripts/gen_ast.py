import json
from dataclasses import dataclass
import inflection
import itertools
from enum import Enum

with open("sqlite.ungram.json", "r") as f:
    UNGRAMMAR = json.loads(f.read())

structs: dict[str, "RustStruct"] = {}
enums: dict[str, "RustEnum"] = {}


@dataclass
class RustStructMethod:
    search_kind: str
    return_type: str
    method_name: str
    child_idx: int | None

    def __str__(self):
        if self.search_kind == "node" and self.return_type in enums:
            self.search_kind = "alt"

        if self.search_kind == "alt":
            return f"""
                pub fn {self.method_name} (&self, ast: &'a SqliteUntypedAst) -> Option<{self.return_type}<'a>> {{
                     self
                        .inner
                        .children(ast)
                        .map(|it| {self.return_type}::try_cast(it))
                        .flatten()
                        .next()
                }}    
                """

        if self.search_kind == "node":
            suffix = ".next()" if self.child_idx is None else f".nth({self.child_idx})"
            return f"""
            pub fn {self.method_name} (&self, ast: &'a SqliteUntypedAst) -> Option<{self.return_type}<'a>> {{
                self.inner
                    .find_children(ast, SqliteTreeKind::{self.return_type})
                    .map({self.return_type}::cast){suffix}
            }}
            """

        if self.search_kind == "token":
            suffix = ".next()" if self.child_idx is None else f".nth({self.child_idx})"
            return f"""
            pub fn {self.method_name} (&self, ast: &'a SqliteUntypedAst) -> Option<&'a SqliteToken> {{
                self.inner
                    .find_children(ast, SqliteTokenKind::{self.return_type})
                    .map(|it| &it.token_child().unwrap().token){suffix}
            }}
            """
        if self.search_kind == "token_text":
            suffix = ".next()" if self.child_idx is None else f".nth({self.child_idx})"
            return f"""
            pub fn {self.method_name} (&self, ast: &'a SqliteUntypedAst) -> Option<&'a SqliteToken> {{
                self.inner.children(ast).filter_map(|it| {{
                    if it.token_child().is_some_and(|it| it.token.text_matches("{clean_token(self.return_type)}")) {{
                        Some(&it.token_child().unwrap().token)
                    }} else {{
                        None
                    }}
                }}){suffix}
            }}
            """

        if self.search_kind == "token_set":
            suffix = ".next()" if self.child_idx is None else f".nth({self.child_idx})"

            return f"""
            pub fn {self.method_name} (&self, ast: &'a SqliteUntypedAst) -> Option<&'a SqliteToken> {{
                self.inner.children(ast).filter_map(|it| {{
                    if it.token_kind().is_some_and(|it| {self.return_type.removeprefix("$")}.contains(it)) {{
                        Some(&it.token_child().unwrap().token)
                    }} else {{
                        None
                    }}
                }}){suffix}
            }}
            """

        if self.search_kind == "rep":
            if self.return_type in structs:
                return f"""
                pub fn {self.method_name} (&self, ast: &'a SqliteUntypedAst) -> impl Iterator<Item = {self.return_type}<'a>> {{
                    self.inner
                        .find_children(ast, SqliteTreeKind::{self.return_type})
                        .map({self.return_type}::cast)
                }}
                """
            elif self.return_type in enums:
                return f"""
                pub fn {self.method_name} (&self, ast: &'a SqliteUntypedAst) -> impl Iterator<Item = {self.return_type}<'a>> {{
                    self.inner
                        .find_children(ast, SqliteTreeKind::{self.return_type})
                        .map(|it| it.tree_child().unwrap().children(ast))
                        .flatten()
                        .map(|it| {self.return_type}::try_cast(it))
                        .flatten()
                    }}
                """
            else:
                raise Exception("Unknown return type for rep")

        raise Exception("Unknown return kind")


def clean_token(x: str) -> str:
    return x.removeprefix("KW_").removeprefix("#text:")


@dataclass
class RustEnumField:
    kind: str
    name: str

    def as_enum_variant(self):
        cleaned = self.removeprefix("KW_").removeprefix("#text:")

        return inflection.camelize(cleaned)

    def as_token_kind(self):
        return self.removeprefix("#text:")


@dataclass
class RustEnum:
    enum_name: str
    kind: str
    enum_fields: list[RustEnumField]

    def __str__(self) -> str:
        res = f"pub enum {inflection.camelize(self.enum_name)}<'a> {{"
        for field in self.enum_fields:
            enum_variant_name = clean_token(field.name)
            if self.kind == "token":
                res += f"    {enum_variant_name}(&'a SqliteToken), "
            elif self.kind == "node":
                res += f"    {enum_variant_name}({enum_variant_name}<'a>),"
        res += "}"

        res += f"impl<'a> {inflection.camelize(self.enum_name)}<'a> {{ pub fn try_cast(node: &'a SqliteNode) -> Option<Self> {{"
        res += "match node {"
        for field in enum.enum_fields:
            if field.kind == "token":
                res += f"""
                    SqliteNode::Token(TokenChild {{token: token @ SqliteToken {{ kind: SqliteTokenKind::{field.name}, .. }}, .. }}) => return Some(Self::{clean_token(field.name)}(&token)),
                    """
            elif field.kind == "token_text":
                res += f"""
                    SqliteNode::Token(tk_child) if tk_child.token.text_matches("STORED") => return Some(Self::{clean_token(field.name)}(&tk_child.token)),
                    """
            elif field.kind == "token_set":
                res += f"""
                    SqliteNode::Token(tk_child) if  {field.name.removeprefix("$")}.contains(tk_child.token.kind) => return Some(Self::{clean_token(field.name)}(&tk_child.token)),
                    """
            elif field.kind == "node":
                res += f"""
                    SqliteNode::Tree(TreeChild {{ kind: SqliteTreeKind::{field.name}, ..}}) => return Some(Self::{field.name}({field.name}::cast(node))),
                    """
            else:
                raise Exception("Unsupported enum field kind")
        res += """
                _ => return None
                }}}
        """
        return res


@dataclass
class RustStruct:
    struct_name: str
    struct_impl: list[RustStructMethod]

    def __str__(self):
        inner_text = "inner" if len(self.struct_impl) > 0 else "_inner"
        res = f"pub struct {self.struct_name}<'a> {{ {inner_text} : &'a TreeChild }}"

        res += f"impl<'a> {self.struct_name}<'a> {{"
        res += f"""
            pub fn cast(node: &'a SqliteNode) -> Self {{
                assert!(matches!(node.tree_kind(), Some(SqliteTreeKind::{self.struct_name})));
                Self {{ {inner_text}: node.tree_child().unwrap() }}
            }}
        """
        for m in self.struct_impl:
            res += str(m)
        res += "}\n"

        return res


def make_rust_item(rule_name, rule):
    if "alt" in rule:
        enum = make_rust_enum(rule_name, rule)
        enums[enum.enum_name] = enum
    elif "seq" in rule:
        struct = make_rust_struct(rule_name, rule)
        structs[struct.struct_name] = struct
    else:
        struct = make_rust_struct(rule_name, {"seq": [rule]})
        structs[struct.struct_name] = struct


def make_rust_struct(struct_name, rule):
    assert "seq" in rule
    methods = []

    skip_nodes = set()

    sub_rules: list = rule["seq"]

    while len(sub_rules) != 0:
        sub_rule = sub_rules.pop(0)

        if "token" in sub_rule:
            continue
        elif "node" in sub_rule and sub_rule["node"] in skip_nodes:
            continue
        elif "label" in sub_rule or "node" in sub_rule:
            method = make_rust_method(sub_rule)
            if method.search_kind == "rep":
                skip_nodes.add(method.return_type)

            methods.append(method)
        # Flatten sub seq rules
        elif "seq" in sub_rule:
            for r in sub_rule["seq"]:
                sub_rules.insert(0, r)
        else:
            print(sub_rule)
            raise Exception("Unsupported rule")

    def remove_methods_of_skipped_nodes(m):
        return not (m.search_kind == "node" and m.return_type in skip_nodes)

    list(filter(remove_methods_of_skipped_nodes, methods))

    return RustStruct(
        struct_name=inflection.camelize(struct_name),
        struct_impl=list(filter(remove_methods_of_skipped_nodes, methods)),
    )


def make_rust_method(rule):
    child_idx = None

    if "node" in rule:
        method_name = inflection.underscore(rule["node"])
        search_kind = "node"
        return_type = inflection.camelize(rule["node"])
    elif "label" in rule:
        label_parts = rule["label"].split("__")
        method_name = inflection.underscore(label_parts[0])

        if len(label_parts) == 2:
            child_idx = ord(label_parts[1]) - ord("a")

        rule = rule["rule"]

        if "alt" in rule:
            enum = make_rust_enum(method_name, rule)
            enums[enum.enum_name] = enum
            search_kind = "alt"
            return_type = inflection.camelize(method_name)
        elif "token" in rule:
            if rule["token"].startswith("$"):
                search_kind = "token_set"
            elif rule["token"].startswith("#text:"):
                search_kind = "token_text"
            else:
                search_kind = "token"
            return_type = rule["token"]
        elif "node" in rule:
            search_kind = "node"
            return_type = inflection.camelize(rule["node"])
        elif "rep" in rule:
            search_kind = "rep"

            if "node" in rule["rep"]:
                return_type = inflection.camelize(rule["rep"]["node"])
            elif "seq" in rule["rep"]:
                return_type = next(filter(lambda x: "node" in x, rule["rep"]["seq"]))[
                    "node"
                ]
            else:
                raise Exception("Unsupported rule")
        else:
            print(rule)
            raise Exception("Unsupported rule")

    return RustStructMethod(
        search_kind=search_kind,
        return_type=return_type,
        method_name=method_name,
        child_idx=child_idx,
    )


def make_rust_enum(enum_name: str, rule):
    assert "alt" in rule

    rule_kinds = set(
        itertools.chain.from_iterable(map(lambda x: x.keys(), rule["alt"]))
    )
    assert len(rule_kinds) <= 2
    assert len(rule_kinds.difference(["node", "token"])) == 0

    enum_fields = []
    for kind, name in itertools.chain.from_iterable(
        map(lambda x: x.items(), rule["alt"])
    ):
        if name.startswith("$"):
            kind = "token_set"
        elif name.startswith("#text:"):
            kind = "token_text"

        kind = kind if not name.startswith("$") else "token_set"
        enum_fields.append(RustEnumField(kind=kind, name=name))

    return RustEnum(
        enum_name=inflection.camelize(enum_name),
        kind=list(rule_kinds)[0],
        enum_fields=enum_fields,
    )


def remove_opt(ungrammar):
    def helper(rule):
        if "opt" in rule:
            return helper(rule["opt"])
        if "seq" in rule:
            return {"seq": [helper(r) for r in rule["seq"]]}
        elif "alt" in rule:
            return {"alt": [helper(r) for r in rule["alt"]]}
        elif "rep" in rule:
            return {"rep": helper(rule["rep"])}
        elif "label" in rule:
            return {"label": rule["label"], "rule": helper(rule["rule"])}
        elif "token" in rule or "node" in rule:
            return rule
        else:
            raise Exception("Rule kind not handled")

    result = {}
    for rule_name, rule in ungrammar.items():
        result[rule_name] = helper(rule)

    return result


def dst(grammar):
    stack = []
    root = grammar["File"]
    stack.append(root)

    while len(stack) != 0:
        node = stack.pop()

        print(node)

        if "label" in node:
            stack.append(node["rule"])
        elif "opt" in node:
            stack.append(node["opt"])
        elif "rep" in node:
            stack.append(node["rep"])
        elif "seq" in node:
            for child in reversed(node["seq"]):
                stack.append(child)
        elif "alt" in node:
            for child in reversed(node["alt"]):
                stack.append(child)
        elif "node" in node:
            stack.append(grammar[node["node"]])


def dst2(tree):
    stack = []
    stack.append(tree)

    while len(stack) != 0:
        node = stack.pop()

        if isinstance(node, dict):
            print(list(node.keys())[0])
        else:
            print(node)

        if isinstance(node, list):
            raise Exception("unreachable")
        elif isinstance(node, dict):
            children = list(node.values())[0]
            for child in reversed(children):
                stack.append(child)


def find_fallback_marker(stack: list):
    lenr = len(stack)
    for i in reversed(range(len(stack))):
        print(f"idx: {i}, ungram_stack_len: {lenr}")
        if isinstance(stack[i], list) and stack[i][0] == "rep-marker":
            print(f"idx: {i}, ungram_stack_len: {lenr}")
            return (i - 1, stack[i][2], stack[i][3])
        elif isinstance(stack[i], list) and stack[i][0] == "opt-marker":
            return (i - 1, stack[i][1], stack[i][2])
        elif isinstance(stack[i], list) and stack[i][0] == "alt-marker":
            stack[i][2] -= 2
            new_marker = stack[i][2]
            if new_marker > stack[i][1]:
                return (new_marker, stack[i][3], stack[i][4])
            else:
                continue


def find_alt_begin(stack: list):
    for i in reversed(range(len(stack))):
        if isinstance(stack[i], list) and stack[i][0] == "alt-marker":
            return i


def both(tree):
    ast_stack = [tree]
    ungram_stack = [{"node": "File"}]

    while len(ast_stack) != 0 or len(ungram_stack) != 0:
        print("============")
        print(ast_stack)
        print(ungram_stack)
        ungram_node = ungram_stack[-1]
        print(ungram_node)

        if isinstance(ungram_node, list) and ungram_node[0] == "rep-marker":
            ungram_stack.append(ungram_node[1])
            continue
        elif isinstance(ungram_node, list) and ungram_node[0] == "alt-marker":
            ungram_stack.pop()
            continue
        elif isinstance(ungram_node, list) and ungram_node[0] == "opt-marker":
            ungram_stack.pop()
            continue
        # if isinstance(ungram_node, str) and ungram_node in ["seq-marker"]:
        #     ungram_stack.pop()
        #     continue
        if ungram_node == "alt-item-begin":
            idx = find_alt_begin(ungram_stack)
            assert idx is not None
            ungram_stack = ungram_stack[:idx]
            continue

        ungram_stack.pop()

        if "label" in ungram_node:
            ungram_stack.append(ungram_node["rule"])
        elif "opt" in ungram_node:
            if len(ast_stack) == 0:
                continue

            marker = ["opt-marker", len(ast_stack) - 1, ast_stack[-1]]
            ungram_stack.append(marker)
            ungram_stack.append(ungram_node["opt"])
        elif "rep" in ungram_node:
            if len(ast_stack) == 0:
                continue
            ungram_stack.append(
                ["rep-marker", ungram_node["rep"], len(ast_stack) - 1, ast_stack[-1]]
            )
            ungram_stack.append(ungram_node["rep"])
        elif "seq" in ungram_node:
            # ungram_stack.append("seq-marker")
            for child in reversed(ungram_node["seq"]):
                ungram_stack.append(child)
        elif "alt" in ungram_node:
            marker = [
                "alt-marker",
                len(ungram_stack),
                len(ungram_stack),
                len(ast_stack) - 1,
                ast_stack[-1],
            ]
            ungram_stack.append(marker)
            for child in reversed(ungram_node["alt"]):
                marker[2] += 2
                ungram_stack.append("alt-item-begin")
                ungram_stack.append(child)
        elif "node" in ungram_node:
            ast_node = ast_stack[-1] if len(ast_stack) > 0 else None

            if (
                isinstance(ast_node, dict)
                and list(ast_node.keys())[0] == ungram_node["node"]
            ):
                print("OK", ungram_node["node"])
                ungram_stack.append(UNGRAMMAR[ungram_node["node"]])
                ast_stack.pop()
                for child in reversed(list(ast_node.values())[0]):
                    ast_stack.append(child)

                # if ungram_stack[-1] == "alt-item-begin":
                #     idx = find_alt_begin(ungram_stack)
                #     assert idx is not None
                #     ungram_stack = ungram_stack[:idx]
            else:
                marker = find_fallback_marker(ungram_stack)
                if marker is not None:
                    (ungram_idx, ast_stack_idx, ast_node) = marker
                    ungram_stack = ungram_stack[: ungram_idx + 1]
                    print(ungram_stack, ungram_idx)
                    ast_stack = ast_stack[:ast_stack_idx]
                    if not ast_stack and not ungram_stack:
                        break
                    ast_stack.append(ast_node)
                    continue
                else:
                    raise Exception(ast_node, ungram_node)
        elif "token" in ungram_node:
            ast_node = ast_stack[-1]

            if isinstance(ast_node, str) and ast_node == ungram_node["token"]:
                print("OK", ast_node)
                ast_stack.pop()
                # if ungram_stack[-1] == "alt-item-begin":
                #     idx = find_alt_begin(ungram_stack)
                #     assert idx is not None
                #     ungram_stack = ungram_stack[:idx]
            else:
                marker = find_fallback_marker(ungram_stack)
                if marker is not None:
                    (ungram_idx, ast_stack_idx, ast_node) = marker
                    ungram_stack = ungram_stack[: ungram_idx + 1]
                    ast_stack = ast_stack[:ast_stack_idx]
                    if not ast_stack and not ungram_stack:
                        break

                    ast_stack.append(ast_node)
                    continue
                else:
                    raise Exception(ast_node, ungram_node)

    if len(ungram_stack) == 0 and len(ast_stack) != 0:
        print(ungram_stack)
        print(ast_stack)
        raise Exception("Unmatched input")


def right_sibling(parent, child):
    if not isinstance(parent, list):
        return None

    for idx, c in enumerate(parent):
        if id(c) == id(child) and (idx + 1) < len(parent):
            return parent[idx]


if __name__ == "__main__":
    # UNGRAMMAR = remove_opt(UNGRAMMAR)

    actual_tree = {
        "File": [
            {
                "Statement": [
                    {
                        "StatementWithCte": [
                            {
                                "SelectStmt": [
                                    {
                                        "SelectCore": [
                                            {
                                                "TraditionalSelect": [
                                                    "KW_SELECT",
                                                    {
                                                        "ResultColumnList": [
                                                            {
                                                                "ResultColumn": [
                                                                    {
                                                                        "ResultColumnAll": [
                                                                            "*"
                                                                        ]
                                                                    }
                                                                ]
                                                            }
                                                        ],
                                                    },
                                                    {
                                                        "FromClause": [
                                                            "KW_FROM",
                                                            {
                                                                "TableOrSubquery": [
                                                                    {
                                                                        "QualifiedTableName": [
                                                                            {
                                                                                "FullTableName": [ {
                                                                                    "TableName": [
                                                                                        "$NAME"
                                                                                    ]
                                                                                }
                                                                                ]
                                                                            }
                                                                        ]
                                                                    }
                                                                ]
                                                            }
                                                        ]
                                                    }
                                                ]
                                            }
                                        ]
                                    }
                                ]
                            }
                        ]
                    }
                ],
            },
            ";"
        ]
    }

    both(actual_tree)
    # for rule_name, rule in UNGRAMMAR.items():
    #     make_rust_item(rule_name, rule)

    # print(
    #     """
    #     pub use crate::{NodeId, SqliteNode, SqliteTokenKind, SqliteTreeKind, SqliteUntypedAst, TokenChild, TreeChild, SqliteToken};
    #     use crate::grammar::common::*;
    #     """
    # )
    # for enum in enums.values():
    #     print(str(enum))

    # for struct in structs.values():
    #     print(str(struct))
