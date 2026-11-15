// ECharts with only the parts the charts use, to keep the bundle small.

import { LineChart } from "echarts/charts";
import {
    DataZoomComponent,
    GridComponent,
    LegendComponent,
    MarkAreaComponent,
    MarkLineComponent,
    TooltipComponent,
} from "echarts/components";
import * as echarts from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";

echarts.use([
    LineChart,
    GridComponent,
    LegendComponent,
    TooltipComponent,
    MarkLineComponent,
    MarkAreaComponent,
    DataZoomComponent,
    CanvasRenderer,
]);

export { echarts };
