<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
  import init, { exampleCall } from "backend";
  import { Layout } from "svelte-utils/two_column_layout";
  import type { Map } from "maplibre-gl";
  import { LineLayer, GeoJSON, MapLibre } from "svelte-maplibre";
  import type { FeatureCollection } from "geojson";
  import bbox from "@turf/bbox";
  import { parse as parseWkt } from "wkt";
  import { twoLines } from "./examples";

  let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";
  let map: Map | undefined;

  let inputWkt = twoLines;

  $: inputGj = parseLines(inputWkt);
  $: zoomTo(map, inputGj);

  function parseLines(wkt: string): FeatureCollection {
    try {
      let gj = parseWkt(wkt);
      gj.type = "FeatureCollection";
      gj.features = gj.geometries.map((geometry) => {
        return {
          type: "Feature",
          geometry,
          properties: {},
        };
      });
      delete gj.geometries;

      if (
        gj.features.length != 2 ||
        gj.features[0].geometry.type != "LineString" ||
        gj.features[1].geometry.type != "LineString"
      ) {
        throw new Error(`Need exactly 2 LineStrings`);
      }
      return gj;
    } catch (err) {
      window.alert(err);
      return {
        type: "FeatureCollection",
        features: [],
      };
    }
  }

  function zoomTo(map: Map | undefined, gj: FeatureCollection) {
    if (gj.features.length > 0) {
      map?.fitBounds(bbox(gj) as [number, number, number, number], {
        animate: false,
        padding: 10,
      });
    }
  }
</script>

<Layout>
  <div slot="left">
    <h1>GeoRust tester</h1>

    <textarea bind:value={inputWkt} />
  </div>

  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style={`https://api.maptiler.com/maps/dataviz/style.json?key=${maptilerApiKey}`}
      standardControls
      hash
      bind:map
    >
      <GeoJSON data={inputGj} generateId>
        <LineLayer
          paint={{
            "line-width": 5,
            "line-color": ["case", ["==", ["id"], 0], "red", "blue"],
          }}
        />
      </GeoJSON>
    </MapLibre>
  </div>
</Layout>
