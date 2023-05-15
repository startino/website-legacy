<script lang="ts">
	let scroll: any;
	let carouselSpeed = 1;
</script>

<svelte:window bind:scrollY={scroll} />
<div class="w-screen">
	<div class="inline-block">
		<div class="box bg-blue-500" />
		<div class="box bg-green-500" />
		<div class="box bg-yellow-500" />
		<div class="box bg-amber-500" />
		<div class="box bg-orange-500" />
		<div class="box bg-pink-500" />
		<div class="box bg-red-500" />
		<div class="box bg-white" />
	</div>
</div>

<style>
	.box {
		@apply h-24 w-48 rounded-lg m-3;
	}
</style>
