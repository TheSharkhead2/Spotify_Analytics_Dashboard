<script lang="ts">
    import Authorize from '../lib/authorization.svelte';
    import RecentlyPlayedAnalytics from '../lib/recently_played_analytics.svelte';
    import PlaylistList from '../lib/PlaylistList.svelte';
    import { AUTHENTICATED } from '../store';

    let authenticated_local: boolean;

    // update authenticated state locally when it is authenticated
    AUTHENTICATED.subscribe(value => {
        authenticated_local = value;
    });

</script>    

<h1>This is app</h1>
{#if authenticated_local === false}
<Authorize />
{:else}
<h2>Authenticated!</h2>
<RecentlyPlayedAnalytics />

<PlaylistList />
{/if}

<style>
    :global(:root) {
       --background-color: #222222;
       --text-color: #e6e6e6;
       --component-background-color: #454545;
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
</style>