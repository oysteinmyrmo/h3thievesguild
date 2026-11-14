/** One game's statistics over time. Every `values` row has one entry per day in `days`. */
export interface Dataset {
    map: MapInfo;
    /**
     * Every day from the first save to the last, in order: the x-axis. Days without a save
     * have `file: null` and `null` values, so the charts show the gap.
     */
    days: Day[];
    /** Runs of days without a save. Empty for a complete series. */
    missingDays: DayRange[];
    /** Ask the user whether to go on when set: days are missing. `null` for a complete series. */
    missingDaysWarning: string | null;
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
    /** Day number in the game, from 1. */
    day: number;
    month: number;
    week: number;
    /** 1-7. */
    dayOfWeek: number;
    /** Such as "1-2-3" for month 1, week 2, day 3. */
    label: string;
    /** The save file this day comes from, or `null` if there is no save for the day. */
    file: string | null;
}

/** A run of consecutive days. */
export interface DayRange {
    /** First day number. */
    first: number;
    /** Last day number, the same as `first` for a single day. */
    last: number;
    /** Such as "Month 1, Week 1, Day 4 to Month 1, Week 2, Day 1". */
    label: string;
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
    kind: "noSaves" | "unreadable" | "mixedGames" | "duplicateDays" | "tooManyDays";
    details?:
        | { file: string; reason: string }[]
        | { mapName: string; mapSize: string; underground: boolean; files: string[] }[]
        | { date: string; files: string[] }[]
        | { days: number; first: { date: string; files: string[] }; last: { date: string; files: string[] } };
}
