<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';

	const VARIANTS = {
		default: 'bg-primary text-800 hover:bg-cream-500 hover:ring-cream-100 hover:text-black',
		outline: 'border-gray-400 bg-cream-200',
		secondary: '',
		dark: '',
		danger: ''
	} as const;

	type variantTypes = keyof typeof VARIANTS;

	const SIZES = {
		default: 'h-8 gap-1.5 px-2.5',
		icon: 'size-8',
		'icon-lg': 'size-9'
	} as const;

	type sizeTypes = keyof typeof SIZES;

	interface Props extends HTMLButtonAttributes {
		variant?: variantTypes;
		size?: sizeTypes;
		class?: string;
		children?: Snippet;
	}

	const {
		variant = 'default',
		size = 'default',
		class: className = '',
		children,
		...rest
	}: Props = $props();
</script>

<button
	type="button"
	class={`inline-flex items-center justify-center rounded transition-all
    ${VARIANTS[variant]} ${SIZES[size]}
    cursor-pointer disabled:pointer-events-none disabled:opacity-50
    ${className}`}
	{...rest}
>
	{@render children?.()}
</button>
