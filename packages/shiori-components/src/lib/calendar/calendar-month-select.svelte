<script lang="ts">
	import { Calendar as CalendarPrimitive } from 'bits-ui';
	import { cn, type WithoutChildrenOrChild } from '$lib/utils.js';
	import ChevronDownIcon from '@lucide/svelte/icons/chevron-down';
	let {
		ref = $bindable(null),
		class: className,
		value,
		onchange,
		...restProps
	}: WithoutChildrenOrChild<CalendarPrimitive.MonthSelectProps> = $props();
</script>

<span
	class={cn(
		'rounded-md border-input shadow-xs has-focus:border-ring has-focus:ring-ring/50 relative flex border has-focus:ring-[3px]',
		className
	)}
>
	<CalendarPrimitive.MonthSelect
		bind:ref
		class="inset-0 bg-background dark:bg-popover dark:text-popover-foreground absolute opacity-0"
		{...restProps}
	>
		{#snippet child({ props, monthItems, selectedMonthItem })}
			<select {...props} {value} {onchange}>
				{#each monthItems as monthItem (monthItem.value)}
					<option
						value={monthItem.value}
						selected={value !== undefined
							? monthItem.value === value
							: monthItem.value === selectedMonthItem.value}
					>
						{monthItem.label}
					</option>
				{/each}
			</select>
			<span
				class="gap-1 rounded-md ps-2 pe-1 text-sm font-medium [&>svg]:size-3.5 [&>svg]:text-muted-foreground flex h-(--cell-size) items-center select-none"
				aria-hidden="true"
			>
				{monthItems.find((item) => item.value === value)?.label || selectedMonthItem.label}
				<ChevronDownIcon class={cn('size-4', className)} />
			</span>
		{/snippet}
	</CalendarPrimitive.MonthSelect>
</span>
