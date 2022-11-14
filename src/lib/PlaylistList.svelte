<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';

    import VirtualList from 'svelte-tiny-virtual-list';;

    import { STYLE } from '../store';

    // function to insure correct typing when requesting playlist data from backend 
    async function getUserPlaylists(): Promise<Array<[string, string, string]>> {
        let raw_playlists: Array<[string, string, string]> = await invoke('get_all_playlist_tiles')
            .then((data) => {
                if (Array.isArray(data) 
                && data.every(element => Array.isArray(element) 
                && element.every(element => typeof element === "string"))) { // confirm outer element is an array, all elements within Array are also arrays (tuples), and elements inside that are strings
                    return data;
                } else {
                    console.log("invalid data from backend");
                    
                    let dummy: Array<[string, string, string]> = [["", "", ""]]; // dummy array to insure type coherence 

                    return dummy;
                }
            })
        return raw_playlists;
    }

    let user_playlists: Promise<Array<[string, string, string]>> = getUserPlaylists(); // call to backend to get data 

</script>

<div class=playlistlist>
    <table class=playlistlist>
        <tr>
            {#await user_playlists then playlists}
                {#each playlists as playlist, _}
                    <td>
                        <div class="playlistTile">  
                            <svg width=450 height=200>
                                
                                <!-- <text x=200 y=45 class=playlistTitleText> {playlist[0]} </text> -->
                                <foreignObject width=160 height=160 x=20 y=20>
                                    <img class="playlistPreviewImage" src={playlist[2]} alt={"Playlist cover image"}/>
                                </foreignObject>

                                <foreignObject x=200 y=5 width=230 height=160>
                                    <p class=playlistTitleText> {playlist[0]} </p>
                                    <div class=playlistDescText>
                                        <p class=playlistDescText> {playlist[1]} </p>
                                    </div>
                                    
                                </foreignObject>
                            </svg> 
                        </div>
                    </td>
                {/each}
            {/await}
        </tr>
    </table>
</div>

<style>
    .playlistTitleText {
        color: var(--text-color);
        font-family: "Open Sans", sans-serif;
        font-weight: normal;
        font-size: 20px;
        overflow: hidden;
        text-overflow: ellipsis;
        word-wrap: break-word;
        
    }

    p.playlistDescText {
        color: var(--text-color);
        font-family: "Open Sans", sans-serif;
        font-weight: normal;
        font-size: 12px;
        overflow: visible;
        overflow-wrap: break-word;
        word-wrap: break-all;
    }
    div.playlistDescText {
        width: 200px;
        height: 100;
        border: 2px solid red;
    }

    div.playlistlist {
        margin-top: 20px;
        margin-bottom: 20px; 
        overflow: auto;
        white-space: nowrap;
        width: 100%;
        display: inline-block;
        flex-wrap: nowrap;
        overflow-x: auto;
    }

    div.playlistlist::-webkit-scrollbar
    {
    height:7px;

    }
    div.playlistlist::-webkit-scrollbar-track
    {
        border-radius: 20px;

    }
    div.playlistlist::-webkit-scrollbar-thumb
    {
        background-color: var(--accent-color);
        border-radius: 20px;

    }

    .playlistPreviewImage {
        width: 160px;
        height: 160px;
        border-radius: 20px;
    }

    .playlistTile {
        margin: 20px;
        background-color: var(--component-background-color);
        border-radius: 35px;
    }
</style>