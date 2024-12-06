<script lang="ts">
  import { SplitComponent } from "svelte-utils/two_column_layout";
  import { isPoint } from "svelte-utils/map";
  import { lineInterpolatePoint } from "backend";
  import { LineLayer, CircleLayer, GeoJSON } from "svelte-maplibre";
  import type { FeatureCollection, Feature, LineString } from "geojson";
  import { parse as parseWkt } from "wkt";
  import { twoLines } from "./examples";
  import EditLine from "./EditLine.svelte";
  import type { Map } from "maplibre-gl";
  import { zoomTo } from "./common";

  export let map: Map;

  let useMercator = true;

  let inputWkt = twoLines;
  let line: Feature<LineString> | undefined;
  let fraction = 0.5;

  $: parseLine(inputWkt);
  $: gj = makeGj(line, fraction);

  function makeGj(
    line: Feature<LineString> | undefined,
    fraction: number,
  ): FeatureCollection {
    let gj: FeatureCollection = {
      type: "FeatureCollection" as const,
      features: [],
    };
    if (line) {
      gj.features.push(line);
      try {
        gj.features.push(
          JSON.parse(lineInterpolatePoint(line, fraction, useMercator)),
        );
      } catch (err) {
        window.alert(err);
      }
    }
    return gj;
  }

  function parseLine(inputWkt: string) {
    line = undefined;

    try {
      let wkt = parseWkt(inputWkt);
      let features = wkt.geometries.map((geometry) => {
        return {
          type: "Feature",
          geometry,
          properties: {},
        };
      });

      if (features.length == 0 || features[0].geometry.type != "LineString") {
        throw new Error(`Need a LineString`);
      }

      line = features[0];

      zoomTo(map, makeGj(line, fraction));
    } catch (err) {
      window.alert(err);
    }
  }
</script>

<SplitComponent>
  <div slot="sidebar">
    <textarea bind:value={inputWkt} />

    <label>
      <input type="checkbox" bind:checked={useMercator} />
      Convert to Euclidean using Mercator projection
    </label>

    <label>
      <input type="range" min="0" max="1" step=".01" bind:value={fraction} />

      Fraction: {fraction}
    </label>
  </div>

  <div slot="map">
    <GeoJSON data={gj} generateId>
      <LineLayer
        paint={{
          "line-width": 5,
          "line-color": ["case", ["==", ["id"], 0], "red", "blue"],
        }}
      />

      <CircleLayer
        filter={isPoint}
        paint={{ "circle-radius": 20, "circle-color": "green" }}
      />
    </GeoJSON>

    {#if line}
      <EditLine bind:f={line} />
    {/if}
  </div>
</SplitComponent>
