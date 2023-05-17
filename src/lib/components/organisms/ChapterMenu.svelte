<script lang="ts">
	import { cubicOut } from 'svelte/easing';
	import { fade, slide } from 'svelte/transition';
	import { circOut, circInOut } from 'svelte/easing';

	let handleExpandClick = () => (isExpanded = !isExpanded);

	const animateIn = {
		delay: 100,
		duration: 700,
		easing: circOut
	};

	const animateOut = {
		delay: 100,
		duration: 500,
		easing: circOut
	};

	let isExpanded: boolean = false;
	let selectedChapter = 1;

	const chapters = [
		{
			chapterNumber: 1,
			title: 'Welcome'
		},
		{
			chapterNumber: 2,
			title: 'Design'
		},
		{
			chapterNumber: 3,
			title: 'Develop'
		},
		{
			chapterNumber: 4,
			title: 'Host'
		},
		{
			chapterNumber: 5,
			title: 'Marketing'
		}
	];
</script>

<div class="fixed z-40 flex-none w-full bottom-0 p-6">
	<button
		on:click={handleExpandClick}
		class="flex flex-row gap-6 max-w-xs bg-surface-variant-light dark:bg-surface-variant-dark rounded-xl p-4 text-left items-center"
	>
		<div class="h-10 w-8 bg-white/50 self-end divide-black/30 dark:divide-white" />

		<div class="flex flex-col">
			<ul
				class="divide-y-1 divide-black/10 dark:divide-white/10 space-y-4"
				in:fade={animateIn}
				out:fade={animateOut}
			>
				{#if isExpanded}
					{#each chapters as chapter}
						<li class="" in:slide={animateIn} out:slide={animateOut}>
							<a on:click={() => (selectedChapter = chapter.chapterNumber)} href="#{chapter.title}">
								{#if chapter.chapterNumber === selectedChapter}
									<h1 class="body-small text-left text-primary-light dark:text-primary-dark">
										Chapter 0{chapter.chapterNumber}
									</h1>
									<h1 class="body-large text-left font-bold">{chapter.title}</h1>
								{:else}
									<h1 class="body-small text-left">Chapter 0{chapter.chapterNumber}</h1>
									<h1 class="body-large text-left font-bold">{chapter.title}</h1>
								{/if}
							</a>
						</li>
					{/each}
				{:else}
					<div class="flex flex-col">
						<h1 class="title-small text-primary-light dark:text-primary-dark">
							Chapter 0{chapters[selectedChapter - 1].chapterNumber}
						</h1>
						<h1 class="title-medium font-bold">{chapters[selectedChapter - 1].title}</h1>
					</div>
				{/if}
			</ul>
		</div>
	</button>
</div>
