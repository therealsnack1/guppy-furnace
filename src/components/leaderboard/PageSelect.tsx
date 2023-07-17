import { Button } from "@chakra-ui/react";

type PageSelectProps = {
  selectedPage: number;
  totalPages: number | undefined;
  onSelect: (newSelectedPage: number) => void;
};

const PageSelect = ({
  selectedPage,
  totalPages,
  onSelect,
}: PageSelectProps) => {
  const getPageButtonsToRender = () => {
    const pageButtons = [selectedPage];

    if (selectedPage > 0) {
      pageButtons.unshift(selectedPage - 1);
    }

    if (totalPages !== undefined) {
      if (totalPages - 1 > selectedPage) {
        for (
          var i = selectedPage + 1;
          i < totalPages && pageButtons.length < 3;
          i++
        ) {
          pageButtons.push(i);
        }
      } else if (totalPages > 2) {
        pageButtons.unshift(selectedPage - 2);
      }
    }

    return pageButtons;
  };

  const pageButtonsToRender = getPageButtonsToRender();

  return (
    <>
      {totalPages != null &&
        pageButtonsToRender.map((value) => {
          return (
            <Button
              variant={selectedPage == value ? "selectedPage" : "page"}
              onClick={() => onSelect(value)}
              key={Math.random()}
            >
              {value + 1}
            </Button>
          );
        })}
    </>
  );
};

export default PageSelect;
