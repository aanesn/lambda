<script lang="ts">
	import Analytics from "$lib/assets/analytics.svg?raw"
	import Chevrons from "$lib/assets/chevrons.svg?raw"
	import Domains from "$lib/assets/domains.svg?raw"
	import Logomark from "$lib/assets/logomark.svg?raw"
	import Logout from "$lib/assets/logout.svg?raw"
	import Plus from "$lib/assets/plus.svg?raw"
	import Projects from "$lib/assets/projects.svg?raw"
	import type { User } from "$lib/bindings"
	import * as Avatar from "$lib/components/avatar"
	import * as DropdownMenu from "$lib/components/dropdown-menu"
	import * as Sidebar from "$lib/components/sidebar"
	import { apiUrl } from "$lib/utils"
	import { page } from "$app/state"

	let { user }: { user: User | null } = $props()

	const navItems = [
		{
			title: "New project",
			href: "/dashboard/new-project",
			icon: Plus
		},
		{
			title: "Projects",
			href: "/dashboard/projects",
			icon: Projects
		},
		{
			title: "Domains",
			href: "/dashboard/domains",
			icon: Domains
		},
		{
			title: "Analytics",
			href: "/dashboard/analytics",
			icon: Analytics
		}
	]

	const dropdownMenuItems = [
		{
			title: "Log out",
			href: `${apiUrl}/auth/logout`,
			icon: Logout
		}
	]
</script>

<Sidebar.Root>
	<Sidebar.Header>
		<div
			class="flex h-12 w-full items-center rounded-xl p-2 transition-[width,height,padding] group-data-[state=collapsed]:size-8! group-data-[state=collapsed]:p-1.5!"
		>
			{@html Logomark}
		</div>
	</Sidebar.Header>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.Menu>
				{#each navItems as { title, href, icon }}
					<Sidebar.MenuItem>
						<Sidebar.MenuButton
							tooltipContent={title}
							isActive={page.url.pathname.startsWith(href)}
						>
							{#snippet child({ props })}
								<a {...props} {href}>
									{@html icon}
									<span>{title}</span>
								</a>
							{/snippet}
						</Sidebar.MenuButton>
					</Sidebar.MenuItem>
				{/each}
			</Sidebar.Menu>
		</Sidebar.Group>
	</Sidebar.Content>
	<Sidebar.Footer>
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<DropdownMenu.Root>
					<DropdownMenu.Trigger>
						{#snippet child({ props })}
							<Sidebar.MenuButton size="lg" {...props}>
								<Avatar.Root class="size-8">
									<Avatar.Image src={user?.avatar_url} alt="avatar" />
									<Avatar.Fallback>
										{user?.name.charAt(0).toUpperCase()}
									</Avatar.Fallback>
								</Avatar.Root>
								<div class="grid flex-1 text-start text-sm leading-tight">
									<span class="truncate font-medium">{user?.name}</span>
									<span class="truncate text-xs">{user?.email}</span>
								</div>
								{@html Chevrons}
							</Sidebar.MenuButton>
						{/snippet}
					</DropdownMenu.Trigger>
					<DropdownMenu.Content class="w-(--bits-dropdown-menu-anchor-width)" side="top">
						{#each dropdownMenuItems as { title, href, icon }}
							<DropdownMenu.Item>
								{#snippet child({ props })}
									<a {...props} {href}>
										{@html icon}
										<span>{title}</span>
									</a>
								{/snippet}
							</DropdownMenu.Item>
						{/each}
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Footer>
</Sidebar.Root>
