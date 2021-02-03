import React from 'react'

export type Context1Type = {
  setLogin: React.Dispatch<React.SetStateAction<boolean>>
  loggedIn: boolean
}

export const Context1 = React.createContext<Partial<Context1Type>>({
  loggedIn: false,
})
