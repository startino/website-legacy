<script lang="ts">
	let scroll: any;
	let carouselSpeed = 1;
</script>

<svelte:window bind:scrollY={scroll} />
<div id="carousel-parent" class="w-full overflow-hidden relative h-42 flex flex-nowrap">
	<div
		class="h-full z-10 bg-gradient-to-r from-surface-light dark:from-surface-dark to-transparent w-20 left-0 absolute"
	/>
	<div
		class="h-full z-10 bg-gradient-to-l from-surface-light dark:from-surface-dark to-transparent w-20 right-0 absolute"
	/>
	{#each Array(3) as repeated_so_that_it_can_cycle}
		<div class="logo-carousel">
			<div class="box bg-yellow-500">1</div>
			<div class="box bg-emerald-500">1</div>
			<div class="box bg-black">1</div>
			<div class="box bg-green-500">1</div>
			<div class="box bg-pink-500">1</div>
			<div class="box bg-white">1</div>
		</div>
	{/each}
</div>

<style>
	.box {
		@apply h-32 w-56 rounded-lg mx-6 flex-shrink-0;
	}

	.logo-carousel {
		@apply flex flex-nowrap flex-row animate-carouselSlide py-3;
	}
</style>
