// Pages are addressed by the URL hash, so Back works and views can be bookmarked.

export type Route = { page: "saves"; stat?: string } | { page: "calculators" };

export function parseRoute(hash: string): Route {
    const parts = hash.replace(/^#\/?/, "").split("/").filter(Boolean).map(decodeURIComponent);
    if (parts[0] === "calculators") {
        return { page: "calculators" };
    }
    if (parts[0] === "saves" && parts[1]) {
        return { page: "saves", stat: parts[1] };
    }
    return { page: "saves" };
}

export function routeHash(route: Route): string {
    if (route.page === "calculators") {
        return "#/calculators";
    }
    return route.stat ? `#/saves/${encodeURIComponent(route.stat)}` : "#/saves";
}

/** The current route, following the URL hash. */
class Router {
    route = $state<Route>(parseRoute(location.hash));

    constructor() {
        addEventListener("hashchange", () => {
            this.route = parseRoute(location.hash);
            // A new view starts at the top, as a new page would.
            scrollTo(0, 0);
        });
    }

    go(route: Route): void {
        location.hash = routeHash(route);
    }
}

export const router = new Router();
