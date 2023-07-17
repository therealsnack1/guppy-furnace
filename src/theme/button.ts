
const primaryDisabledStyle = {
  bg: "lightGray",
        color: 'white',
        fontSize: 18,
        fontWeight: 450,
        padding: 6,
        width: "200px",
        opacity: 1,
        filter: "none",
}

const buttonStyles = {
  variants: {
    primary: {
      bg: "brandGreen",
      color: 'white',
      fontSize: 18,
      fontWeight: 450,
      padding: 6,
      width: "200px",
      transition: "filter 0.5s ease-out 100ms",
      filter: "drop-shadow(0px 0px 5px #3CCD64)",
      _hover: {
          bg: "brandGreen",
          color: 'white',
          filter: "drop-shadow(0px 0px 20px #3CCD64)",
          _disabled: {
            ...primaryDisabledStyle
          }
      },
      _disabled: {
      ...primaryDisabledStyle
      }
  },
  secondary: {
    bg: "darkTransparentGray",
    color: "lightGray",
    borderRadius: "100px",
    fontWeight: 500,
    minWidth: "125px",
    _hover: {
      color: "white",
    },
    _disabled: {
      color: "white",
      borderColor: "brandGreen",
      borderWidth: "2px",
    }
  },
  navbar: {
    outline: "none",
    color: "white",
    fontFamily: `'Lato', sans-serif`,
    fontSize: 16,
    fontWeight: 500,
    backgroundColor: "transparent",
    padding: 2,
    _hover: {
      color: "brandGreen"
    }
  },
  modal: {
    bg: "darkGray",
    fontWeight: 500,
    minWidth: "70%",
    paddingLeft: 6,
    paddingRight: 6,
    borderColor: "transparent",
    borderWidth: "2px",
    _hover: {
      color: "brandGreen",
      borderColor: "brandGreen",
      borderWidth: "2px",
    }
  },
  page: {
    fontFamily: `'Inter', sans-serif`,
    fontSize: "16px",
    fontWeight: 400,
    color: "#BBBBBB",
    background: "transparentGray",
    borderRadius: "50%",
    height: "45.22px",
    width: "45.25px",
    _disabled: {
      color: "white",
      background: "brandGreen",
      opacity: 1
    }
  },
  selectedPage: {
    fontFamily: `'Inter', sans-serif`,
    fontSize: "16px",
    fontWeight: 400,
    color: "white",
      background: "brandGreen",
      opacity: 1,
    borderRadius: "50%",
    height: "45.22px",
    width: "45.25px",
  }
  }
}

export default buttonStyles;