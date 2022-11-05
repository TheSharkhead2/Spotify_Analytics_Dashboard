<script lang="ts">
	import { select, selectAll } from 'd3-selection';
	import { axisBottom, axisLeft } from 'd3-axis';

	// export let width: number;
	export let width: number;
	export let margin: number;
    export let marginleftright: number;
    export let average: number;
	export let scale: any;

	let transform: string = `translate(${[margin, margin]})`;;
	let g: any;

	$: {
		select(g).selectAll('*').remove();

		let axis: any = axisLeft(scale).tickValues([average]).tickSize(-(width-marginleftright));

        select(g).call(axis).call(g => g.select(".domain").remove()).selectAll("path").style("fill", "none");
	}
</script>

<g class='averageaxis' bind:this={g} {transform} />

<style>
    .averageaxis {
        stroke-dasharray: 1 3;
    }
</style>