<svelte:head>
    
    <title>Futino Home</title>
</svelte:head>

<script lang="ts">
    import ThemeButton from '$lib/components/molecules/ThemeButton.svelte';
    import Logo from '$lib/components/atoms/Logo.svelte';
    import Button from '$lib/components/atoms/Button.svelte';

</script>

<div class="min-h-screen bg-gradient-to-r from-background-light to-secondary-light/5 dark:from-background-dark dark:to-secondary-dark/5">
    <div class="fixed top-0 z-40 flex-none w-full transition-colors duration-500 backdrop-blur bg-primary-container-light/5 dark:bg-primary-container-dark/5 text-primary-container-on-light dark:text-primary-container-on-dark">
        <div class="py-4 mx-5 border-b border-secondary-light/10 dark:border-secondary-dark/10">
            <div class="flex relative items-center md:px-10 lg:px-20 xl:px-40 px-auto md:px-auto">
                <a class="flex overflow-hidden px-3 gap-3 justify-center" href="/">
                    <Logo />
                    <p class="invisible pt-0.5 text-lg font-bold sm:visible">Futino</p>
                </a>

                <div class="flex items-center ml-auto">
                    <nav class="text-sm font-semibold leading-6">
                        <ul class="flex m-auto space-x-8">
                            <li>
                                <a class="inline hover:text-tertiary-light dark:hover:text-tertiary-dark"
                                href="/">
                                Home
                                </a>
                            </li>
                            <li>
                                <a class="inline hover:text-tertiary-light dark:hover:text-tertiary-dark"
                                href="/">
                                Contact
                                </a>
                            </li>
                            <li>
                                <a class="inline hover:text-tertiary-light dark:hover:text-tertiary-dark"
                                href="/">
                                About
                                </a>
                            </li>
                            <li class="border-l border-secondary-light/20 dark:border-secondary-dark/20"/>
                            <li>
                                <ThemeButton />
                            </li>
                        </ul>
                    </nav>
                </div>
            </div>
        </div>
    </div>

    <div class="py-40 px-4 border-b shadow-2xl sm:px-6 md:px-8 border-primary-light/40 dark:border-primary-dark/40">
        <div class="grid justify-items-center space-y-6 mx-auto max-w-6xl">
            <h1 class="text-6xl">
            Futino
            </h1>

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

    <footer class="pt-20 p-6 border-t border-secondary-light/10 dark:border-secondary-dark/10">
        <div class="sm:flex sm:justify-between sm:items-center md:px-10 lg:px-20 xl:px-40 px-auto md:px-auto">
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
        <hr class="my-6 sm:mx-auto lg:my-8 border-secondary-light/10 dark:border-secondary-dark/10" />
        <span class="block text-sm sm:text-center text-background-on-light dark:text-background-on-dark">
            Unpublished © 2023 
            <a href="/" class="text-tertiary-light dark:text-tertiary-dark hover:underline hover:text-tertiary-light dark:hover:text-tertiary-light">
                Futino™
            </a>
        </span>
    </footer>
</div>
