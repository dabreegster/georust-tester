import type { Map } from "maplibre-gl";
import type { FeatureCollection } from "geojson";
import bbox from "@turf/bbox";

export function zoomTo(map: Map, gj: FeatureCollection) {
  if (gj.features.length > 0) {
    map.fitBounds(bbox(gj) as [number, number, number, number], {
      animate: false,
      padding: 10,
    });
  }
}
