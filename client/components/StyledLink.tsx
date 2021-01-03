import Link from 'next/link'
import { ChevronRight } from 'react-feather'

interface StyledLinkProps {
  href: string
  title: string
  className?: string
}

export default function StyledLink(props: StyledLinkProps) {
  return (
    <Link href={props.href}>
      <a
        className={`group flex items-center hover:text-gray-500 transition-colors text-sm ${props.className}`}
      >
        {props.title}
        <ChevronRight
          size={16}
          className={`group-hover:translate-x-0.5 transform transition-transform`}
        />
      </a>
    </Link>
  )
}
