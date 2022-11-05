<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import { AUTHENTICATED } from '../store';

    async function authorize() {
        await invoke('authorization_codeflow'); // authenticate user
        AUTHENTICATED.set(true); // update authenticated state to true
    }
</script>

<div>
    <button on:click={authorize}>Authorize</button>
</div>
