<!-- 
    Component based on Axis component found here: https://github.com/TomFevrier/svelte-d3-demo/blob/master/src/Axis.svelte
-->
<script lang="ts">
	import { select, selectAll } from 'd3-selection';
	import { axisBottom, axisLeft } from 'd3-axis';

	// export let width: number;
	export let height: number;
	export let margin: number;
	export let position: string;
	export let scale: any;

	let transform: any;
	let g: any;

	$: {
		select(g).selectAll('*').remove();

		let axis: any;
		switch(position) {
			case 'bottom':
				axis = axisBottom(scale).tickSizeOuter(0).ticks(0);
				transform = `translate(${[margin, margin+height]})`;
				break;
			case 'left':
				axis = axisLeft(scale).tickSizeOuter(0).ticks(0);
                transform = `translate(${[margin, margin]})`
		}

		select(g).call(axis);
	}
</script>

<g class='axis' bind:this={g} {transform} />
