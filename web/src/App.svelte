<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
  import init, { compareLines } from "backend";
  import { Layout } from "svelte-utils/two_column_layout";
  import type { Map } from "maplibre-gl";
  import { LineLayer, GeoJSON, MapLibre } from "svelte-maplibre";
  import type { FeatureCollection, Feature, LineString } from "geojson";
  import bbox from "@turf/bbox";
  import { parse as parseWkt } from "wkt";
  import { twoLines } from "./examples";
  import EditLine from "./EditLine.svelte";
  import { onMount } from "svelte";

  let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";
  let map: Map | undefined;
  let loaded = false;

  let inputWkt = twoLines;
  let line1: Feature<LineString> | undefined;
  let line2: Feature<LineString> | undefined;

  onMount(async () => {
    await init();
    loaded = true;
  });

  $: parseLines(inputWkt);
  $: gj = makeGj(line1, line2);
  // TODO Don't trigger when editing a line by markers
  $: zoomTo(map, gj);

  function makeGj(
    line1: Feature<LineString> | undefined,
    line2: Feature<LineString> | undefined,
  ) {
    return {
      type: "FeatureCollection" as const,
      features: line1 && line2 ? [line1, line2] : [],
    };
  }

  function parseLines(inputWkt: string) {
    line1 = undefined;
    line2 = undefined;

    try {
      let wkt = parseWkt(inputWkt);
      let features = wkt.geometries.map((geometry) => {
        return {
          type: "Feature",
          geometry,
          properties: {},
        };
      });

      if (
        features.length != 2 ||
        features[0].geometry.type != "LineString" ||
        features[1].geometry.type != "LineString"
      ) {
        throw new Error(`Need exactly 2 LineStrings`);
      }

      line1 = features[0];
      line2 = features[1];
    } catch (err) {
      window.alert(err);
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

    {#if loaded && line1 && line2}
      <pre>{JSON.stringify(
          JSON.parse(compareLines(line1, line2)),
          null,
          "  ",
        )}</pre>
    {/if}
  </div>

  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style={`https://api.maptiler.com/maps/dataviz/style.json?key=${maptilerApiKey}`}
      standardControls
      bind:map
    >
      <GeoJSON data={gj} generateId>
        <LineLayer
          paint={{
            "line-width": 5,
            "line-color": ["case", ["==", ["id"], 0], "red", "blue"],
          }}
        />
      </GeoJSON>

      {#if line1}
        <EditLine bind:f={line1} />
      {/if}
      {#if line2}
        <EditLine bind:f={line2} />
      {/if}
    </MapLibre>
  </div>
</Layout>
