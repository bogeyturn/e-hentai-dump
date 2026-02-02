export interface NavItem {
    label: string
    to: string
}

export const bountyNavItems: NavItem[] = [
    {label: "Bounty List", to: "/bounty"},
    {label: "Most Wanted Standard Bounties", to: "/bounty?act=tops"},
    {label: "Most Wanted Translation Bounties", to: "/bounty?act=topt"},
    {label: "Most Wanted Editing Bounties", to: "/bounty?act=tope"},
    {label: "Post New Bounty", to: "/bounty_post"},
]
export const configNavItems: NavItem[] = [
    {label: "Overview", to: "/home"},
    {label: "My Stats", to: "/stats"},
    {label: "My Settings", to: "/uconfig"},
    {label: "My Tags", to: "/mytags"},
    {label: "Hentai@Home", to: "/hentaiathome"},
    {label: "Donations", to: "https://e-hentai.org/bitcoin.php"},
    {label: "Hath Perks", to: "/hathperks"},
    {label: "Hath Exchange", to: "/exchange?t=hath"},
    {label: "GP Exchange", to: "/exchange?t=gp"},
    {label: "Credit Log", to: "/logs?t=credits"},
    {label: "Karma Log", to: "/logs?t=karma"}
]

export const uploadNavItems: NavItem[] = [
    {label: "Gallery List", to: "/upload/manage"},
    {label: "Manage Folders", to: "/upload/managefolders"},
    {label: "Create New Gallery", to: "/upload/managegallery?act=new"},
]
