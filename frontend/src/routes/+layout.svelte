<script>
    import '../app.css';
    import NavButton from 'components/NavButton.svelte';
    import NextTask from 'components/NextTask.svelte';
    import ToolBox from 'components/ToolBox.svelte';

    function toggleDarkMode() {
        window.document.body.classList.toggle('dark');
    }
</script>

<div class="flex flex-row justify-center w-full h-screen">
    <div class="flex flex-col max-w-7xl h-full justify-center">
        <slot />
    </div>
</div>
