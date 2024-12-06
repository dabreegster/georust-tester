<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
  import init from "backend";
  import {
    Layout,
    mapContents,
    sidebarContents,
  } from "svelte-utils/two_column_layout";
  import type { Map } from "maplibre-gl";
  import { onMount } from "svelte";
  import { MapLibre } from "svelte-maplibre";
  import CompareLineStrings from "./CompareLineStrings.svelte";
  import LineInterpolatePoint from "./LineInterpolatePoint.svelte";

  let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";
  let map: Map | undefined;
  let loaded = false;
  let mode = "CompareLineStrings";

  let modes = ["CompareLineStrings", "LineInterpolatePoint"];

  onMount(async () => {
    await init();
    loaded = true;
  });

  let sidebarDiv: HTMLDivElement | undefined;
  let mapDiv: HTMLDivElement | undefined;
  $: if (sidebarDiv && $sidebarContents) {
    sidebarDiv.innerHTML = "";
    sidebarDiv.appendChild($sidebarContents);
  }
  $: if (mapDiv && $mapContents) {
    mapDiv.innerHTML = "";
    mapDiv.appendChild($mapContents);
  }
</script>

<Layout>
  <div slot="left">
    <h1>GeoRust tester</h1>

    <label>
      Mode:
      <select bind:value={mode}>
        {#each modes as mode}
          <option value={mode}>{mode}</option>
        {/each}
      </select>
    </label>

    <div bind:this={sidebarDiv} />
  </div>

  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style={`https://api.maptiler.com/maps/dataviz/style.json?key=${maptilerApiKey}`}
      standardControls
      bind:map
    >
      <div bind:this={mapDiv} />

      {#if loaded && map}
        {#if mode == "CompareLineStrings"}
          <CompareLineStrings {map} />
        {:else if mode == "LineInterpolatePoint"}
          <LineInterpolatePoint {map} />
        {/if}
      {/if}
    </MapLibre>
  </div>
</Layout>
