<script>
    import '../app.css';
    import NavButton from 'components/NavButton.svelte';
    import NextTask from 'components/NextTask.svelte';
    import ToolBox from 'components/ToolBox.svelte';

    function toggleDarkMode() {
        window.document.body.classList.toggle('dark');
    }
</script>

<div class="flex flex-row w-full h-screen">
    <aside
        class="flex flex-col top-0 left-0 w-96 h-full bg-emerald-900 dark:bg-slate-950 items-start justify-between"
    >
        <section class="flex flex-col w-full space-y-4">
            <NavButton title="Current task" />
            <NavButton href="/" title="All tasks" />
        </section>
        <section class="flex flex-col w-full space-y-4">
            <NavButton onClick={toggleDarkMode} title="Toggle dark mode" />
        </section>
    </aside>
    <div class="flex flex-col w-full h-full bg-gray-100 dark:bg-slate-900">
        <header class="flex flex-row">
            <NextTask />
            <ToolBox />
        </header>
        <slot />
    </div>
</div>
