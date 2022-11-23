<script lang="ts">
    import Icon from './Icon.svelte';
    import { PLAYLIST_DISPLAY } from '../store';

    export let playlist_id: string | boolean;
    
    function handle_back_click() {
        PLAYLIST_DISPLAY.set(false);
    }

    

</script>


<div class=main-container>
    <button class=back-button on:click={handle_back_click}>
        <Icon name="left_arrow" width={"30px"} height={"30px"}/>
    </button>
    <p class=playlist-title>Test</p>

    <p class="basic-text"> {playlist_id} </p>
</div>

<style>
    .back-button {
        background-color: var(--background-color);
        border: none;
        border-radius: 50%;
        padding: 5px 7px;
        color: var(--text-color);
    }

    .back-button:hover {
        background-color: #2c2c2c;
    }

    .playlist-title {
        color: var(--text-color);
        font-size: 1.5rem;
        display: inline-block;
        padding-left: 15px;
        font-family: "Open Sans", sans-serif;
        font-weight: normal;
        font-size: 30px;
    }

    .main-container {
        padding-top: 0px;
        background-image: url({});
    }
</style>