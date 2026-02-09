<script lang="ts">
	import Analytics from "$lib/assets/analytics.svg?raw"
	import Chevron from "$lib/assets/chevron.svg?raw"
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

	const displayName = $derived(user?.name || user?.email || "User")

	const sidebarMenuLinks = [
		{ title: "New project", href: "/dashboard/new-project", icon: Plus },
		{ title: "Projects", href: "/dashboard/projects", icon: Projects },
		{ title: "Domains", href: "/dashboard/domains", icon: Domains },
		{ title: "Analytics", href: "/dashboard/analytics", icon: Analytics }
	]

	const dropdownMenuLinks = [{ title: "Log out", href: `${apiUrl}/auth/logout`, icon: Logout }]
</script>

<Sidebar.Root>
	<Sidebar.Header>
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<Sidebar.MenuButton intent="secondary" size="lg">
					{@html Logomark}
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Header>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.Menu>
				{#each sidebarMenuLinks as { title, href, icon }}
					<Sidebar.MenuItem>
						<Sidebar.MenuButton
							tooltipContent={title}
							isActive={page.url.pathname.startsWith(href)}
						>
							{#snippet child({ props })}
								<a {href} {...props}>
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
							<Sidebar.MenuButton
								intent="outline"
								size="lg"
								{...props}
								class="rounded-full group-data-[state=collapsed]:p-0!"
							>
								<Avatar.Root>
									<Avatar.Image src={user?.avatar_url} alt="avatar" />
									<Avatar.Fallback>
										{displayName[0].toUpperCase()}
									</Avatar.Fallback>
								</Avatar.Root>
								<span class="flex-1 truncate">{displayName}</span>
								<div class="mr-2 -rotate-90">
									{@html Chevron}
								</div>
							</Sidebar.MenuButton>
						{/snippet}
					</DropdownMenu.Trigger>
					<DropdownMenu.Content class="w-(--bits-dropdown-menu-anchor-width)" side="top">
						{#each dropdownMenuLinks as { title, href, icon }}
							<DropdownMenu.Item>
								{#snippet child({ props })}
									<a {href} {...props}>
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
