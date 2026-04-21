<script lang="ts">
	import { Calendar as CalendarPrimitive } from 'bits-ui';
	import { cn, type WithoutChildrenOrChild } from '$lib/utils.js';
	import ChevronDownIcon from '@lucide/svelte/icons/chevron-down';
	let {
		ref = $bindable(null),
		class: className,
		value,
		...restProps
	}: WithoutChildrenOrChild<CalendarPrimitive.YearSelectProps> = $props();
</script>

<span
	class={cn(
		'rounded-md border-input shadow-xs has-focus:border-ring has-focus:ring-ring/50 relative flex border has-focus:ring-[3px]',
		className
	)}
>
	<CalendarPrimitive.YearSelect
		bind:ref
		class="inset-0 dark:bg-popover dark:text-popover-foreground absolute opacity-0"
		{...restProps}
	>
		{#snippet child({ props, yearItems, selectedYearItem })}
			<select {...props} {value}>
				{#each yearItems as yearItem (yearItem.value)}
					<option
						value={yearItem.value}
						selected={value !== undefined
							? yearItem.value === value
							: yearItem.value === selectedYearItem.value}
					>
						{yearItem.label}
					</option>
				{/each}
			</select>
			<span
				class="gap-1 rounded-md ps-2 pe-1 text-sm font-medium [&>svg]:size-3.5 [&>svg]:text-muted-foreground flex h-(--cell-size) items-center select-none"
				aria-hidden="true"
			>
				{yearItems.find((item) => item.value === value)?.label || selectedYearItem.label}
				<ChevronDownIcon class={cn('size-4', className)} />
			</span>
		{/snippet}
	</CalendarPrimitive.YearSelect>
</span>
