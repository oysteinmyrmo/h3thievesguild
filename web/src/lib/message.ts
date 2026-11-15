// The parser's messages are lines of text: a heading, then "- " lines naming files or days,
// then perhaps a closing line. This splits them for display.

export interface MessageParts {
    title: string;
    items: string[];
    /** Lines after the list. */
    after: string[];
}

export function splitMessage(message: string): MessageParts {
    const [first = "", ...rest] = message.split("\n");
    const items = rest.filter((line) => line.startsWith("- ")).map((line) => line.slice(2));
    const after = rest.filter((line) => !line.startsWith("- ") && line.trim() !== "");
    // Messages start in lower case; as a heading, the first letter is capitalised.
    const title = first.charAt(0).toUpperCase() + first.slice(1);
    return { title, items, after };
}
