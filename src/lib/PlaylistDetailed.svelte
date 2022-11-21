<script lang="ts">
    import Icon from './Icon.svelte';
    import { PLAYLIST_DISPLAY } from '../store';

    export let playlist_id: string | boolean;
    
    function handle_back_click() {
        PLAYLIST_DISPLAY.set(false);
    }

</script>


<div>
    
    <button class=back-button on:click={handle_back_click}>
        <Icon name="left_arrow" width={"25px"} height={"25px"}/>
    </button>
    <p class="basic-text"> {playlist_id} </p>
</div>

<style>
    .back-button {
        background-color: var(--background-color);
        border: none;
        border-radius: 100px;
        padding: 5px 7px;
        color: var(--text-color);
    }

    .back-button:hover {
        background-color: #2c2c2c;
    }
</style>