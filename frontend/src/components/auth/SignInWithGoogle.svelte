<script lang="ts">
    import { env } from '$env/dynamic/public';
    import { accounts, type CredentialResponse } from 'google-one-tap';
    import { onMount } from 'svelte';

    export let prompt: boolean = false;

    onMount(() => {
        const handleCredentialResponse = async (response: CredentialResponse) => {
            console.log('Encoded JWT ID token: ' + response.credential);
            alert('Logged in');
        };

        accounts.id.initialize({
            client_id: env.PUBLIC_GOOGLE_CLIENT_ID || '',
            callback: handleCredentialResponse
        });

        accounts.id.renderButton(document.getElementById('google-signin-button') as HTMLElement, {
            theme: 'outline',
            size: 'large'
        });

        if (prompt) {
            accounts.id.prompt();
        }
    });
</script>

<svelte:head>
    <script src="https://accounts.google.com/gsi/client" async defer></script>
</svelte:head>

<div id="google-signin-button" />
