<svelte:head>
    
    <title>Futino Home</title>
</svelte:head>

<script lang="ts">
    import Text from '$lib/components/atoms/Text.svelte';
    import Logo from '$lib/components/atoms/Logo.svelte';
    import Button from '$lib/components/atoms/Button.svelte';

    function changeTheme() {
        if (localStorage.theme === 'dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
            localStorage.setItem("theme", "light");
            document.documentElement.classList.remove('dark');
        } else {
            localStorage.setItem("theme", "dark");
            document.documentElement.classList.add('dark');
        }
    }
</script>

<div class="min-h-screen text-black dark:text-white bg-gradient-to-r from-background-light to-secondary-light/5 dark:from-background-dark dark:to-secondary-dark/5">
    <div class="fixed top-0 z-40 flex-none w-full transition-colors duration-500 backdrop-blur bg-surface-light/5 dark:bg-surface-dark/5">
        <div class="py-4 mx-5 border-b border-secondary-light/10 dark:border-secondary-dark/10">
            <div class="flex relative items-center md:px-10 lg:px-20 xl:px-40 px-auto md:px-auto">
                <a class="flex overflow-hidden px-3 gap-3 justify-center" href="/">
                    <Logo />
                    <p class="invisible pt-0.5 text-lg font-bold sm:visible">Futino</p>
                </a>

                <div class="flex items-center ml-auto">
                    <nav class="text-sm font-semibold leading-6 text-background-on-light dark:text-background-on-dark hover:text-primary-light dark:hover:text-primary-dark">
                        <ul class="flex m-auto space-x-8">
                            <li>
                                <a class="inline text-secondary-light dark:text-secondary-dark hover:text-tertiary-light dark:hover:text-tertiary-dark"
                                href="/">
                                Home
                                </a>
                            </li>
                            <li>
                                <a class="inline text-secondary-light dark:text-secondary-dark hover:text-tertiary-light dark:hover:text-tertiary-dark"
                                href="/">
                                Contact
                                </a>
                            </li>
                            <li>
                                <a class="inline text-secondary-light dark:text-secondary-dark hover:text-tertiary-light dark:hover:text-tertiary-dark"
                                href="/">
                                About
                                </a>
                            </li>
                            <li class="border-l border-surface-variant-on-light dark:border-surface-variant-on-dark"/>
                            <li>
                                <button on:click={changeTheme} class="fill-primary-light stroke-primary-light hover:fill-tertiary-light hover:stroke-tertiary-light dark:fill-primary-dark dark:stroke-primary-dark dark:hover:fill-tertiary-dark dark:hover:stroke-tertiary-dark">
                                    <!-- Light mode -->
                                    <span class="dark:hidden">
                                        <svg viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6">
                                            <path d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"/>
                                            <path d="M12 4v1M17.66 6.344l-.828.828M20.005 12.004h-1M17.66 17.664l-.828-.828M12 20.01V19M6.34 17.664l.835-.836M3.995 12.004h1.01M6 6l.835.836"/>
                                        </svg>
                                    </span>
                        
                                    <!-- Dark mode -->
                                    <span class="hidden dark:inline">
                                        <svg viewBox="0 0 24 24" fill="none" class="w-6 h-6">
                                            <path fill-rule="evenodd" clip-rule="evenodd" d="M17.715 15.15A6.5 6.5 0 0 1 9 6.035C6.106 6.922 4 9.645 4 12.867c0 3.94 3.153 7.136 7.042 7.136 3.101 0 5.734-2.032 6.673-4.853Z"/>
                                            <path fill-rule="evenodd" clip-rule="evenodd" d="M17 3a1 1 0 0 1 1 1 2 2 0 0 0 2 2 1 1 0 1 1 0 2 2 2 0 0 0-2 2 1 1 0 1 1-2 0 2 2 0 0 0-2-2 1 1 0 1 1 0-2 2 2 0 0 0 2-2 1 1 0 0 1 1-1Z"/>
                                        </svg>
                                    </span>
                                </button>
                            </li>
                        </ul>
                    </nav>
                </div>
            </div>
        </div>
    </div>

    <div class="py-40 px-4 border-b shadow-2xl sm:px-6 md:px-8 border-primary-light/40 dark:border-primary-dark/40">
        <div class="grid justify-items-center space-y-6 mx-auto max-w-6xl">
            <Text textType={'Display'} textSize={'Medium'}>
            Futino
            </Text>

            <h3 class="text-2xl">
                {"Launch Your Business's Online Presence with Confidence And Trust"}
            </h3>

            <div class="space-x-6">
                <a href="/contact">
                    <Button>
                        <p>Contact Us!</p>
                    </Button>
                </a>
                <a href="/about">
                    <Button>
                        <p>Who Are We?</p>
                    </Button>
                </a>
            </div>
        </div>
    </div>

    <footer class="p-6 rounded-lg shadow border-primary-light/40 dark:border-primary-dark/40">
        <div class="sm:flex sm:justify-between sm:items-center">
                <Logo />
            <ul class="flex flex-wrap items-center mb-6 space-x-6 text-sm  sm:mb-0 text-background-on-light dark:text-background-on-dark">
                <li>
                    <a class="inline text-secondary-light dark:text-secondary-dark hover:text-tertiary-light dark:hover:text-tertiary-dark"
                    href="/">
                    Privacy Policy
                    </a>
                </li>
                <li>
                    <a class="inline text-secondary-light dark:text-secondary-dark hover:text-tertiary-light dark:hover:text-tertiary-dark"
                    href="/">
                    Licensing
                    </a>
                </li>
                <li>
                    <a class="inline text-secondary-light dark:text-secondary-dark hover:text-tertiary-light dark:hover:text-tertiary-dark"
                    href="/">
                    Contact
                    </a>
                </li>
            </ul>
        </div>
        <hr class="my-6 sm:mx-auto lg:my-8 border-primary-light/40 dark:border-primary-dark/40" />
        <span class="block text-sm sm:text-center text-background-on-light dark:text-background-on-dark">{"Unpublished © 2023 "}<a href="#" class="hover:underline text-primary-light hover:text-tertiary-light dark:text-primary-light dark:hover:text-tertiary-light">{"Futino™"}</a>
        </span>
    </footer>
</div>
