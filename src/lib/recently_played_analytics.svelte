<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';
    import * as d3 from 'd3';
    import { line, axisLeft, type ScaleLinear, curveNatural, axisBottom} from 'd3';
    import {select, selectAll} from 'd3-selection';
    import AverageAxis from '../lib/AverageAxis.svelte';

    import Select from 'svelte-select';

    import { STYLE } from "../store";
	import { dataset_dev } from 'svelte/internal';

    let vis: any; // binding div for visualization 

    const MARGIN = {
        top: 40, 
        bottom: 40, 
        right: 40, 
        left: 40
    };

    let width: number;
    let height: number = 600; 

    $: xScale = d3.scaleLinear().domain([0, 50]).range([0, width-MARGIN.left-MARGIN.right]);
    $: yScale = d3.scaleLinear().domain([0, 1]).range([height-MARGIN.bottom-MARGIN.top, 0]);


    // dropdown logic 
    let dropdown_items = [
        {value: 'acousticness', label: 'Acousticness across recent history'},
        {value: 'danceability', label: 'Danceability across recent history'},
        {value: 'energy', label: 'Energy across recent history'},
        {value: 'instrumentalness', label: 'Instrumentalness across recent history'},
        {value: 'liveness', label: 'Liveness across recent history'},
        {value: 'speechiness', label: 'Speechiness across recent history'},
        {value: 'valence', label: 'Valence across recent history'},
    ];

    let dropdown_value = dropdown_items[6];

    function handleDropdownSelect(event: any) {
        dropdown_value = event.detail; // update value
        promise_data = getRecentlyPlayed(); // update data 
    };

    let promise_data: Promise<[Array<number>, number]> = getRecentlyPlayed(); // call to backend to get data

    // call backend to get data and form into Array<number>
    async function getRecentlyPlayed(): Promise<[Array<number>, number]> {
        let valence_data = invoke('get_recent_song_data_feature', {feature: dropdown_value.value})
            .then((data) => {
                let valence_array: Array<number>; 
                let valence_average: number = 0;
                if (Array.isArray(data)) {
                    if (Array.isArray(data[0]) && data[0].every(element => typeof element === "number")) { // check to see if it is Array<number> 
                        valence_array = data[0];
                        valence_average = data[1]; // get average 
                    } else { // if invalid type is returned, return empty list
                        let dummy: Array<number> = []; 
                        valence_array = dummy;
                    }
                } else {
                    let dummy: Array<number> = [];
                    
                    valence_array = dummy;
                }
                let checked_data: [Array<number>, number] = [valence_array, valence_average]; // temp value to ensure type coherence
                return checked_data;
            })
        return valence_data
    };

    async function curveGen(_: any): Promise<string> {
        let gen = line()
        .x((d) => MARGIN.left+xScale(d[0]+1))
        .y((d) => MARGIN.bottom+yScale(d[1]))
        .curve(curveNatural);

        let data = await promise_data; 
        let curve_temp = gen(data[0].entries());

        if (typeof curve_temp === "string") {
            return curve_temp;
        } else {
            let dummy: string = "";
            return dummy;
        }
    }
    $: curve = curveGen([width, dropdown_value]);

    // create y axis 
    let y_axis_transform: string = `translate(${[40, 40]})`; // adjusts position on screen 
    let y_axis: any;

    $: {
        select(y_axis).selectAll('*').remove();

        let axis: any = axisLeft(yScale).tickValues([0.2, 0.5, 0.8]).tickSize(-(width-MARGIN.left-MARGIN.right));

        select(y_axis).call(axis).call(g => g.select(".domain").remove())
            .selectAll("path").style("fill", "none");
    }

    // create extra y axis for average value 
    let average_y_axis_transform: string = `translate(${[40, 40]})`;
    let average_y_axis: any;

    $: {
        select(average_y_axis).selectAll('*').remove();

        let axis: any = axisLeft(yScale).tickValues([]).tickSize(-(width-MARGIN.left-MARGIN.right));

        select(average_y_axis).call(axis).call(g => g.select(".domain").remove()).selectAll("path").style("fill", "none");
    }

</script>

<div class="graph">
    <div class="dropdown"><Select items={dropdown_items} value={dropdown_value} isClearable={false} on:select={handleDropdownSelect}></Select></div>
    <!-- <p class="titletext">Valence across recent history</p> -->
    <div class='scatter-plot' bind:clientWidth={width}>
        
        {#if width}
            <svg width={width} height={height}> 
                <g class='yaxis' bind:this={y_axis} transform={y_axis_transform} />
                {#await curve then curve}
                    <path
                        d={curve}
                        fill="none"
                        stroke={STYLE.data_point_color}
                        opacity=0.8
                    />
                {/await}
                {#await promise_data then data}
                    <AverageAxis width={width} margin={40} marginleftright={MARGIN.left+MARGIN.right} average={data[1]} scale={yScale}/>
                    {#each data[0] as d,i}
                        <circle
                            cx={MARGIN.left+xScale(+i+1)}
                            cy={MARGIN.bottom+yScale(+d)}
                            r={STYLE.data_point_r}
                            fill={STYLE.data_point_color}
                            />
                    {/each} 
                    
                {/await}
            </svg>
        {/if}
    </div>
</div>

<style>
    .yaxis {
        stroke-dasharray: 1 3;
        opacity: 0.4;
    }
    /* .titletext {
        color: var(--text-color);
        font-family: "Open Sans", sans-serif;
        font-weight: normal;
        font-size: 15pt;
        margin-left: 25px;
    } */
    .dropdown {
        --borderRadius: 15px;
        --padding: 20px;
        width: 50%;
        font-family: "Open Sans", sans-serif;
        font-weight: normal;
        font-size: 15pt;
        --background: #5c5c5c;
        --listBackground: #5c5c5c;
        --itemIsActiveBG: #4e4e4e;
        --listBorderRadius: 15px;
        --itemHoverBG: #4e4e4e;
        --listShadow: 0 4px 5px 0 rgba(34, 40, 46, 0.24);
        --listMaxHeight: 400px;
        --borderFocusColor: #7e7e7e;
        --borderHoverColor: #7e7e7e;
        --border: 2px solid #6b6b6b;
    }
</style>