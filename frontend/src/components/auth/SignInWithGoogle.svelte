<script lang="ts">
    import { env } from '$env/dynamic/public';
    import { onMount } from 'svelte';
    import type { CredentialResponse } from 'google-one-tap';

    export let prompt: boolean = false;

    onMount(() => {
        const handleCredentialResponse = (response: CredentialResponse) => {
            console.log('Encoded JWT ID token: ' + response.credential);
            alert('Logged in');
        };

        if (typeof google !== 'undefined' && google.accounts) {
            google.accounts.id.initialize({
                client_id: env.PUBLIC_GOOGLE_CLIENT_ID || '',
                callback: handleCredentialResponse
            });

            google.accounts.id.renderButton(
                document.getElementById('google-signin-button') as HTMLElement,
                {
                    theme: 'outline',
                    size: 'large'
                }
            );

            if (prompt) {
                google.accounts.id.prompt();
            }
        } else {
            console.error('Google One Tap script not loaded');
        }
    });
</script>

<svelte:head>
    <script src="https://accounts.google.com/gsi/client" async defer></script>
</svelte:head>

<div id="google-signin-button" />
