<script lang="ts">
	import Button from '../atoms/Button.svelte';
	import PlanCard from '../molecules/PlanCard.svelte';

	export let plans = [
		{
			label: 'Partnership',
			unit: '',
			cost: '3.8%',
			note: 'of gross revenue',
			features: [
				'50% off Hosting Plans',
				'50% off Marketing Plans',
				'90% off Initial Website Developement',
				'50% off Commissions'
			],
			value: 1
		}
	];

	export let activeTabValue = 0;

	const handleClick = (tabValue: number) => () => (activeTabValue = tabValue);
</script>

<div class="grid space-x-10 space-y-10 place-items-center">
	<h1 class="title-large ">Shooting for the stars?<br/>You'll reach the stars with us.</h1>

	<ul class="flex flex-col xl:flex-row space-y-5 xl:space-x-5 xl:space-y-0">
		{#each plans as plan}
			<li>
				<button
					class="inline-block p-4 rounded-t-lg shadow-xl bg-surface-variant-dark/20 {activeTabValue ===
					plan.value
						? 'active border-2 rounded-xl border-primary-light dark:border-primary-dark transition-all duration-500 scale-110'
						: 'hover:border-2 rounded-xl'}"
					on:click={handleClick(plan.value)}
					type="button"
					role="plan"
				>
					<PlanCard {plan} />
				</button>
			</li>
		{/each}
	</ul>
</div>
