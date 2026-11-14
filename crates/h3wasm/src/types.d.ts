/** One game's statistics over time. Every `values` row has one entry per day in `days`. */
export interface Dataset {
    map: MapInfo;
    /** The days with a save, in order: the x-axis. */
    days: Day[];
    playerStats: StatInfo[];
    heroStats: StatInfo[];
    /** Players in the game on at least one day, in colour order. */
    players: PlayerSeries[];
    /** Heroes owned by a player on at least one day. */
    heroes: HeroSeries[];
}

export interface MapInfo {
    name: string;
    description: string;
    /** S, M, L, XL, H, XH or G. */
    size: string;
    /** Tiles per side. */
    tiles: number;
    underground: boolean;
}

export interface Day {
    /** Day number in the game, from 1. Days without a save are missing. */
    day: number;
    month: number;
    week: number;
    /** 1-7. */
    dayOfWeek: number;
    /** Such as "1-2-3" for month 1, week 2, day 3. */
    label: string;
    /** The save file this day comes from. */
    file: string;
}

export interface StatInfo {
    /** Key in `values`. */
    id: string;
    name: string;
    /** Such as "Resources". */
    group: string;
}

export interface PlayerSeries {
    /** Colour ID, such as "red". */
    id: string;
    /** Such as "Red". */
    name: string;
    /** CSS colour, such as "#f80000". */
    color: string;
    /** First day out of the game after having been in it. */
    eliminatedDay: number | null;
    /** One row per statistic in `playerStats`, `null` while out of the game. */
    values: Record<string, (number | null)[]>;
}

export interface HeroSeries {
    /** Index of the hero record in the save. */
    id: number;
    name: string;
    /** The owner's colour ID on each day, `null` while unowned. */
    owner: (string | null)[];
    /** One row per statistic in `heroStats`, `null` while unowned. */
    values: Record<string, (number | null)[]>;
}

/** Thrown by `DatasetBuilder.finish`. `message` lists every file involved. */
export interface DatasetError extends Error {
    name: "DatasetError";
    kind: "noSaves" | "unreadable" | "mixedGames" | "duplicateDays";
    details?:
        | { file: string; reason: string }[]
        | { mapName: string; mapSize: string; underground: boolean; files: string[] }[]
        | { date: string; files: string[] }[];
}
