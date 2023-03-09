<script lang="ts">
	import Button from '../atoms/Button.svelte';
</script>

<div class="flex flex-col space-y-10 p-7 xl:w-[300px] lg:w-[200px] md:w-[160px] sm:w-[100px]">
	<h1 class="headline-small">
		{$$props.plan.label}
	</h1>
	<h1 class="headline-large">
		${$$props.plan.cost}
	</h1>
	<Button>
		<p class="body-small">Get Started</p>
	</Button>
	<ul class="flex flex-col space-y-2">
		{#each $$props.plan.features as feature}
			<li class="whitespace-pre text-left body-medium">
				{"  •  "} {feature}
			</li>
		{/each}
	</ul>
</div>
