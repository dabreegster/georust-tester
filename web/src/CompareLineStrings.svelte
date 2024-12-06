<script lang="ts">
  import { SplitComponent } from "svelte-utils/two_column_layout";
  import { compareLines } from "backend";
  import { LineLayer, GeoJSON } from "svelte-maplibre";
  import type { FeatureCollection, Feature, LineString } from "geojson";
  import { parse as parseWkt } from "wkt";
  import { twoLines } from "./examples";
  import EditLine from "./EditLine.svelte";
  import type { Map } from "maplibre-gl";
  import { zoomTo } from "./common";

  export let map: Map;

  let useMercator = true;

  let inputWkt = twoLines;
  let line1: Feature<LineString> | undefined;
  let line2: Feature<LineString> | undefined;

  $: parseLines(inputWkt);
  $: gj = makeGj(line1, line2);

  function makeGj(
    line1: Feature<LineString> | undefined,
    line2: Feature<LineString> | undefined,
  ): FeatureCollection {
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

      zoomTo(map, makeGj(line1, line2));
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

    {#if line1 && line2}
      <pre>{JSON.stringify(
          JSON.parse(compareLines(line1, line2, useMercator)),
          null,
          "  ",
        )}</pre>
    {/if}
  </div>

  <div slot="map">
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
  </div>
</SplitComponent>
