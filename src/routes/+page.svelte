<script lang="ts">
    import Authorize from '../lib/authorization.svelte';
    import RecentlyPlayedAnalytics from '../lib/recently_played_analytics.svelte';
    import PlaylistList from '../lib/PlaylistList.svelte';
    import PlaylistDetailed from '../lib/PlaylistDetailed.svelte';
    import { AUTHENTICATED, PLAYLIST_DISPLAY, PLAYLIST_DISPLAY_ID } from '../store';

    let authenticated_local: boolean;

    // update authenticated state locally when it is authenticated
    AUTHENTICATED.subscribe(value => {
        authenticated_local = value;
    });

    let playlist_display_local: boolean; 
    let playlist_display_id_local: string; 

    // update playlist state locally when it is changed 
    PLAYLIST_DISPLAY.subscribe(value => {
        playlist_display_local = value;
    });

    PLAYLIST_DISPLAY_ID.subscribe(value => {
        playlist_display_id_local = value;
    });

</script>    


{#if authenticated_local === false}
    <Authorize />   
{:else}
    {#if playlist_display_local === false}
        <h2>Authenticated!</h2>
        <RecentlyPlayedAnalytics />

        <PlaylistList />
    {:else}
        <PlaylistDetailed playlist_id={playlist_display_id_local} />
    {/if}
{/if}

<style>
    :global(:root) {
       --background-color: #222222;
       --text-color: #e6e6e6;
       --component-background-color: #454545;
       --accent-color: #0798f8;
    }
    :global(body) {
        background-color: var(--background-color);
        color: #e6e6e6;
    }
    :global(#vis) {
        width: 100%;
        height: 100%;
        background-color: var(--background-color);
    }
    :global(.graph) {
        background-color: var(--component-background-color);
        border-radius: 35px;
        padding: 20px;
    }

    :global(html, body) {
        -ms-overflow-style: none;
        scrollbar-width: none;
    }

    :global(html::-webkit-scrollbar, body::-webkit-scrollbar) {
        /* hide scrollbar */
        display: none;
    }

    :global(.basic-text) {
        color: var(--text-color);
        font-family: "Open Sans", sans-serif;
        font-weight: normal;
        font-size: 16px;
        overflow: wrap;
        text-overflow: ellipsis;
        word-wrap: break-word;
    }
</style>