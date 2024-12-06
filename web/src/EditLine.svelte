<script lang="ts">
  import type { Feature, LineString } from "geojson";
  import { Marker } from "svelte-maplibre";

  export let f: Feature<LineString>;

  let pt1 = f.geometry.coordinates[0];
  console.log(pt1);
</script>

{#each f.geometry.coordinates as pt}
  <Marker draggable bind:lngLat={pt}>
    <span class="dot" />
  </Marker>
{/each}

<style>
  .dot {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: flex;
    background-color: grey;
  }

  .dot:hover {
    border: 1px solid black;
    cursor: pointer;
  }
</style>
