// What to show the user for each kind of load error.

import type { LoadError } from "./types";

export interface ErrorView {
    title: string;
    /** One line per problem, each naming the files involved. */
    items: string[];
    hint: string;
}

interface FileReason {
    file: string;
    reason: string;
}
interface GameFiles {
    mapName: string;
    mapSize: string;
    underground: boolean;
    files: string[];
}
interface DayFiles {
    date: string;
    files: string[];
}

export function errorView(error: LoadError): ErrorView {
    switch (error.kind) {
        case "noSaves":
            return { title: "No save files were chosen.", items: [], hint: "Choose the save files of one game." };
        case "unreadable":
            return {
                title: "Some files could not be read as Heroes 3 saves.",
                items: (error.details as FileReason[]).map((f) => `${f.file}: ${f.reason}`),
                hint: "Remove these files, or check that they are HotA 1.8 saves.",
            };
        case "mixedGames":
            return {
                title: "The files come from more than one game.",
                items: (error.details as GameFiles[]).map((g) => {
                    const underground = g.underground ? ", with underground" : "";
                    return `${g.mapName} (${g.mapSize}${underground}): ${g.files.join(", ")}`;
                }),
                hint: "Choose the saves of one game only.",
            };
        case "duplicateDays":
            return {
                title: "There is more than one save for the same day.",
                items: (error.details as DayFiles[]).map((d) => `${d.date}: ${d.files.join(", ")}`),
                hint: "Keep one save per day.",
            };
        case "tooManyDays": {
            const d = error.details as { days: number; first: DayFiles; last: DayFiles };
            return {
                title: `The saves span ${d.days} days, more than a game can.`,
                items: [`First: ${d.first.files.join(", ")} (${d.first.date})`, `Last: ${d.last.files.join(", ")} (${d.last.date})`],
                hint: "One of these saves has a damaged date.",
            };
        }
        case "unexpected":
            return { title: "Something went wrong while reading the saves.", items: [error.message], hint: "" };
    }
}
