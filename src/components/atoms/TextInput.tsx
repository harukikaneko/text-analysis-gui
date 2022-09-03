import React from "react"

interface Props {
    palceholder?: string
    handleOnChange: (e: React.ChangeEvent<HTMLInputElement>) => void
}

export const TextInput: React.FC<Props> = (props) => {
    const handleOnChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        props.handleOnChange(e)
    }

    return (
        <div>
          <input
            id="text-input"
            onChange={handleOnChange}
            placeholder={props.palceholder}
          />
        </div>
    )
}