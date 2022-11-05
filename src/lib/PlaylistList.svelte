<script lang="ts">
    
</script>

<div>
    <ul class="playlistlist">
        {#each Array(3) as _, i}
            <li>{i+1}</li>
        {/each}
    </ul>
</div>

<style>
    ul.playlistlist {
        list-style-type: none;
    }

    ul.playlistlist > li {
        display: inline-block;
    }
</style>