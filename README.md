# Simple daemon to publish Prometheus metrics from a cheap CO2 monitor

This is basically RIIR of <https://github.com/wreiner/officeweather>. Except I'm
too lazy to write a nice web UI, so it instead publishes Prometheus metrics,
which can then be visualized in Grafana. The dashboard is trivial, but you can
find it in `dashboard.json`.

## Additional references

The legwork of figuring out the monitor's protocol is done in
<https://hackaday.io/project/5301-reverse-engineering-a-low-cost-usb-co-monitor/log/17909-all-your-base-are-belong-to-us>. Many thanks for that.
