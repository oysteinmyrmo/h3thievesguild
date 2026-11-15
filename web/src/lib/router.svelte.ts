// The current route, following the URL hash.

import { parseRoute, routeHash, type Route } from "./route";

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
