import React from 'react'
import { Link } from 'react-router-dom'
import { ChevronRight } from 'react-feather'

interface StyledLinkProps {
  href: string
  title: string
  className?: string
}

export default function StyledLink(props: StyledLinkProps) {
  return (
    <Link
      to={props.href}
      className={`
        group
        flex
        items-center
        hover:text-gray-500
        transition-colors
        text-sm
        ${props.className}`}
    >
      {props.title}
      <ChevronRight
        size={16}
        className="
          group-hover:translate-x-0.5
          transform
          transition-transform"
      />
    </Link>
  )
}
