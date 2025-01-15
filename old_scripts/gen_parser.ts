import { snakeCase as sk } from "jsr:@mesqueeb/case-anything";
const snakeCase = (s: string) => sk(s, { keepSpecialCharacters: true })

type UngramJsonNode =
    | { node: string }
    | { token: string }
    | { rep: UngramJsonNode }
    | { seq: UngramJsonNode[] }
    | { alt: UngramJsonNode[] }
    | { opt: UngramJsonNode }
    | { label: string; rule: UngramJsonNode };


type UngramNode =
    | {
        kind: "node";
        name: string;
        child: UngramNode;
        parent: UngramNode | null;
    }
    | { kind: "token"; name: string; parent: UngramNode }
    | {
        kind: "seq" | "alt";
        children: UngramNode[];
        parent: UngramNode;
    }
    | { kind: "opt" | "rep"; child: UngramNode; parent: UngramNode };

const UNGRAMMAR = buildGrammar()


function buildGrammar() {
    const nodesMap = new Map<string, UngramNode>();
    const ungrammarJson: { [key: string]: UngramJsonNode }  = JSON.parse(
        Deno.readTextFileSync("sqlite.ungram.json"),
    );

    const helper = (parent: UngramNode, ungramNode: UngramJsonNode): UngramNode => {
        if ("node" in ungramNode) {
            const name = ungramNode["node"] as string;
            if (nodesMap.get(name)) {
                return nodesMap.get(name)!;
            } else {
                const newNode: UngramNode = {
                    kind: "node",
                    name,
                    parent,
                    child: {} as UngramNode,
                };

                nodesMap.set(name, newNode)
                newNode.child = helper(newNode, ungrammarJson[name]);

                return newNode;
            }
        } else if ("seq" in ungramNode) {
            const newNode: UngramNode = { kind: "seq", parent, children: [] };
            newNode.children = ungramNode.seq.map((it) => helper(newNode, it));

            return newNode;
        } else if ("alt" in ungramNode) {
            const newNode: UngramNode = { kind: "alt", parent, children: [] };
            newNode.children = ungramNode.alt.map((it) => helper(newNode, it));

            return newNode;
        } else if ("opt" in ungramNode) {
            const newNode: UngramNode = {
                kind: "opt",
                parent,
                child: {} as UngramNode,
            };
            newNode.child = helper(newNode, ungramNode["opt"]);

            return newNode;
        } else if ("rep" in ungramNode) {
            const newNode: UngramNode = {
                kind: "rep",
                parent,
                child: {} as UngramNode,
            };
            newNode.child = helper(newNode, ungramNode["rep"]);

            return newNode;
        } else if ("label" in ungramNode) {
            return helper(parent, ungramNode.rule)
        } else if ("token" in ungramNode) {
            return {
                kind: "token",
                parent,
                name: ungramNode.token
            }
        } else {
            throw "Unreachable"
        }
    };

    const root: UngramNode = { kind: "node", name: "File", parent: null, child: {} as UngramNode }

    root.child = helper(root, ungrammarJson["File"])
    
    nodesMap.set("File", root)

    return nodesMap
}



function rightSibling(node: UngramNode) {
    if (node.parent?.kind === "seq") {
        const rightSiblingIdx = node.parent.children.findIndex(n => n === node) + 1

        if (rightSiblingIdx < node.parent.children.length) {
            return node.parent.children[rightSiblingIdx]
        }
    }
}


function tokenCombinations(node: UngramNode): string[][] {
    const MAX_SIZE = 6
    let size = 0

    const visited = new Set<string>()

    const helper = (tokensList: string[][], node: UngramNode) => {
        tokensList = cleanTokenList(tokensList)

        if (node.kind === "node") {
            if (visited.has(node.name)) {
                return tokensList
            } else {
                visited.add(node.name)
            }
        }

        if (node.kind === "token") {
            if (tokensList.length === 0) tokensList.push([])
            tokensList.forEach(l => {
                if (l.length < MAX_SIZE) {
                    l.push(node.name)
                }
            })
            size++
            return tokensList
        } else if (node.kind === "seq") {
            for (const child of node.children) {
                tokensList = helper(tokensList, child)
            }

            return tokensList
        } else if (node.kind === "alt") {
            const original = structuredClone(tokensList)
    
            tokensList = helper(tokensList, node.children[0])

            node.children.slice(1).forEach((it) => {
                const newList = helper(structuredClone(original), it)
                newList.forEach(it => tokensList.push(it))
            })

            return tokensList
        } else if (node.kind === "node") {
            return helper(tokensList, node.child)
        } else if (node.kind === "opt" || node.kind === "rep") {
            const copyList = structuredClone(tokensList)
            tokensList = helper(tokensList, node.child)
            copyList.forEach(it => tokensList.push(it))
    
            return tokensList
        }

        throw Error("Unreachable")
    }

    const tokensList = helper([], node)
    
    return cleanTokenList(tokensList)
}

function cleanToken(tk: string): string {
    const code = tk.charCodeAt(0);
    if (tk === "(" || tk === ")") {
        return `T!['${tk}']`
    }
    if (!(code > 47 && code < 58) && // numeric (0-9)
        !(code > 64 && code < 91) && // upper alpha (A-Z)
        !(code > 96 && code < 123)) { // lower alpha (a-z)
      return `T![${tk}]`
    } else {
        return tk
    }
    
}
function rustCode(node: UngramNode, makeNodeFunctions: boolean): string {
    if (node.kind === "token") {
        return `p.eat_or_error("${node.name}", r);`
    }

    if (node.kind === "node") {
        if (!makeNodeFunctions) {
            return `${snakeCase(node.name)}(p, r);`
        } else {
            return `
            pub fn ${snakeCase(node.name)}(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) {
                let m = p.open();

                ${rustCode(node.child, false)}

                p.close(m, ${node.name});
            }\n`
        }
    }

    if (node.kind === "seq") {
        return node.children.map(it => rustCode(it, false)).join("\n")
    }

    if (node.kind === "opt" || node.kind === "rep") {
        let thisNodeTokenList = tokenCombinations(node)

        // Remove duplicates
        thisNodeTokenList = cleanTokenList(thisNodeTokenList)

        const nextNodes = nextNodePossibilities(node)
        const nextNodeTokenLists: string[][][] = []

        nextNodes.forEach((it, idx) => {
            nextNodeTokenLists[idx] = tokenCombinations(it);
        })

        let finalMatchPatterns = []

        for (const singleValidTokenOrder of thisNodeTokenList) {
            if (nextNodeTokenLists.length === 0) {
                finalMatchPatterns.push([singleValidTokenOrder[0]])
                continue
            }

            for (const allValidTokenOrders of nextNodeTokenLists) {
                const numMaxSimilarTokenOrderCount = Math.max(...allValidTokenOrders.map(it => maxCommonArrayPrefix(it, singleValidTokenOrder)))

                if (numMaxSimilarTokenOrderCount + 1 > singleValidTokenOrder.length) {
                    finalMatchPatterns.push(singleValidTokenOrder.slice(0, numMaxSimilarTokenOrderCount))
                } else {
                    finalMatchPatterns.push(singleValidTokenOrder.slice(0, numMaxSimilarTokenOrderCount + 1))
                }
            }
        }

        finalMatchPatterns = cleanTokenList(finalMatchPatterns)

        let result = node.kind === "opt" ? "if " : "while "
        for (const [i, pattern] of finalMatchPatterns.entries()) {
            console.assert(pattern.length > 0)
            if (i !== 0) result += " || "
            if (pattern.length == 1) {
                if (pattern[0].startsWith("$")) {
                    result += `p.at_any(${cleanToken(pattern[0].replace("$", ""))})`
                } else {
                    result += `p.at(${cleanToken(pattern[0])})`
                }
            } else {
                result +=  `matches!(p.tokens(), [${pattern.map(it => cleanToken(it)).join(", ")}, ..])`
            }
        }
        result += `{ ${rustCode(node.child, false)} }`

        return result
    }

    if (node.kind === "alt") {
        if (node.children.every(it => it.kind === "token")) {
            const items = node.children.map(it => it.name).join(" | ")
            return `p.eat_any_or_error("${items}", r, "Expected on of: ${items}");`
        }

        const nodeTokenLists: string[][][] = node.children.map(it => {
            const thisNodeTokenList: string[][] = tokenCombinations(it)
    
            // Remove duplicates
            return cleanTokenList(thisNodeTokenList)
        })

        const finalMatchPatterns: string[][][] = []
        for (const [i, allValidTokenOrdersOfNode] of nodeTokenLists.entries()) {
            finalMatchPatterns.push([])
            for (const singleValidTokenOrder of allValidTokenOrdersOfNode) {
                for (const [k, allValidTokenOrders] of nodeTokenLists.entries()) {
                    if (i === k) continue;

                    const numMaxSimilarTokenOrderCount = Math.max(...allValidTokenOrders.map(it => maxCommonArrayPrefix(it, singleValidTokenOrder)))
    
                    if (numMaxSimilarTokenOrderCount + 1 > singleValidTokenOrder.length) {
                        finalMatchPatterns[i].push(singleValidTokenOrder.slice(0, numMaxSimilarTokenOrderCount))
                    } else {
                        finalMatchPatterns[i].push(singleValidTokenOrder.slice(0, numMaxSimilarTokenOrderCount + 1))
                    }
                    
                }
            }
            finalMatchPatterns[i] = cleanTokenList(finalMatchPatterns[i])
        }

        let result = ""
        for (const [i, child] of node.children.entries()) {
            if (i !== 0) result += "else "

            for (const [j, pattern] of finalMatchPatterns[i].entries()) {
                if (j === 0) {
                    result += "if "
                } else {
                    result += " || "
                }
                if (pattern.length == 1) {
                    if (pattern[0].startsWith("$")) {
                        result += `p.at_any(${cleanToken(pattern[0].replace("$", ""))})`
                    } else if (pattern[0].startsWith("#text:")) {
                        result += `p.at(IDEN) && p.curr_non_triv_token().is_some_and(|it| it.text_matches("${pattern[0].replace("#text:", "")}"))`
                    } else {
                        result += `p.at(${cleanToken(pattern[0])})`
                    }
                } else {
                    result +=  `matches!(p.tokens(), [${pattern.map(it => cleanToken(it)).join(", ")}])`
                }
            }
            result += `{ ${rustCode(child, false)} }\n`
        }
        const altChildNames = node.children.map(it => {
            if(it.kind === "node" || it.kind === "token") {
                return it.name
            } else {
                return `(${tokenCombinations(it)})`
            }
        })
        result += `else {
            p.advance_with_error(r, "Expected one of ${altChildNames.join(" | ")}");
        }`

        return result
    }

    throw Error("Unreachable")
}

function maxCommonArrayPrefix(a: string[], b: string[]) {
    let i = 0
    while (i < a.length && i < b.length && a[i] === b[i]) i++;
    
    return i
}

function cleanTokenList(list: string[][]) {
    const deduplicatedSet = new Set(list.filter(it => it.length > 0).map(it => it.join("###")))

    return Array.from(deduplicatedSet).map(it => it.split("###"))
}

function nextNodePossibilities(node: UngramNode) {
    const result: UngramNode[] = []

    const findNextNode = (node: UngramNode) => {
        let sib = rightSibling(node)
        while (sib ===  undefined && node.parent !== null ) {
            node = node.parent
            sib = rightSibling(node)
        }

        return sib
    }

    let nextNode = findNextNode(node)
    while (nextNode) {
        result.push(nextNode)
        if (nextNode.kind === "opt" || nextNode.kind === "rep") {
            nextNode = findNextNode(nextNode)
        } else {
            break
        }
    }

    return result
}

Array.from(UNGRAMMAR.values()).reverse().forEach(node => console.log(rustCode(node, true)))
