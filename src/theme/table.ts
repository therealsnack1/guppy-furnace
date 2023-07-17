const tableStyle = {
  variants: {
    simple: {
      th: {
        textTransform: "none",
        fontSize: "16px",
        color: "lightGray",
        fontWeight: 325,
        border: "none",
      },
      td: {
        color: "white",
        border: "none"
      },
      tr: {
        borderBottomColor: "lightTransparentGray",
        borderBottomWidth: "1px",
        '&:last-child': {
          border: "none",
        },
      }
    }
  }
}

export default tableStyle;