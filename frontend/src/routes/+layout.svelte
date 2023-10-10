<script>
    import { page } from '$app/stores';

    import CurrentTask from 'components/CurrentTask.svelte';
    import '../app.css';

    function toggleDarkMode() {
        window.document.body.classList.toggle('dark');
    }

    $: banner = $page.url.pathname !== '/';
</script>

<div class="flex flex-row justify-center w-full h-screen">
    <div class="flex flex-col h-full w-full items-center">
        <CurrentTask {banner} />
        <slot />
    </div>
</div>
