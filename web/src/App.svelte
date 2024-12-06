<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
  import init, { exampleCall } from "backend";
  import { Layout } from "svelte-utils/two_column_layout";
  import type { Map } from "maplibre-gl";
  import {
    LineLayer,
    FillLayer,
    GeoJSON,
    MapLibre,
    hoverStateFilter,
    CircleLayer,
  } from "svelte-maplibre";
  import type {
    FeatureCollection,
    LineString,
    Polygon,
    Geometry,
    GeoJsonProperties,
    Point,
  } from "geojson";
  import bbox from "@turf/bbox";
  import { parse as parseWkt } from "wkt";

  let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

  let map: Map | undefined;
</script>

<Layout>
  <div slot="left">
    <h1>GeoRust tester</h1>
  </div>

  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style={`https://api.maptiler.com/maps/dataviz/style.json?key=${maptilerApiKey}`}
      standardControls
      hash
      bind:map
    ></MapLibre>
  </div>
</Layout>
