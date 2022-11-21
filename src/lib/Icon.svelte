<!-- 
    This component is used to display an icon and is based on code found here: https://stackoverflow.com/questions/66069647/best-way-to-import-svg-icons-into-a-svelte-app
 -->
<script lang="ts">
    const ICONS = [
        {
            box: 24, // icon size
            name: "left_arrow", // icon name
            svg: `<g width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-arrow-left"><line x1="19" y1="12" x2="5" y2="12"></line><polyline points="12 19 5 12 12 5"></polyline></g>` // icon data 
        }
    ]

    export let name: string; // name of the icon
    export let width: string = "1rem";
    export let height: string = "1rem";

    let display_icon: {box: number, name: string, svg: string} = ICONS[0]; // icon data (initially set to the first icon in the array)

    let display_icon_temp = ICONS.find((icon) => icon.name === name) // get icon data

    if (typeof display_icon_temp !== 'undefined') {
        display_icon = display_icon_temp; // set icon data
    }

</script>

<svg
    class={$$props.class}
    width={width}
    height={height}
    viewBox="0 0 {display_icon.box} {display_icon.box}">
    {@html display_icon.svg}
</svg>

<style>
    .icon {
        width: 24px;
        height: 24px;
        stroke: var(--text-color);
        stroke-width: 2;
        stroke-linecap: round;
        stroke-linejoin: round;
        fill: none;
    }
</style>