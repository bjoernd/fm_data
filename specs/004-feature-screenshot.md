## Overview

We are going to implement a new fm_data tool: `fm_image`. The input to this tool will be a screenshot from the game Football Manager. The screenshot will show the data for a specific player. The tool will extract all relevant player attributes from the screenshot using local OCR processing. The tool will take the attribute data and print it in a well defined order.

## Relevant Attributes

We are going to extract the following attributes from the image. The attributes will always appear roughly in the same location and order.

- Name	 (string) -- the player's name
- Age (int) -- the player's age
- Foot (string) -- the player's preferred foot. This will not be available in plain text and I will provide details later.
- Player attributes (list of integers) -- these will be in different order based on player types and I will provide details later.

### Player Preferred Foot

The screenshot will contain a section with the strings "LEFT FOOT" and "RIGHT FOOT". Between these two sections, there will be two colored
shapes in the form of circles. If the left circle has a green or yellow color tone, we will consider the player left-footed. If the right
circle has a green or yellow color tone, we will consider the player right footed. Players can be both left and right footed if both circles are green/yellow. If the color of either circle cannot be clearly identified as green or yellow, the tool should raise an error and abort.

### Player Attributes

Attribute lists differ between goalkeepers and field players. Goalkeepers are detected by the presence of a "GOALKEEPING" section in the screenshot. If this section is not present, the player is considered a field player.

If an attribute is not given in the screenshot, consider it 0. However, all required attributes must be successfully extracted from the image. If any required attribute cannot be found or read, the tool should report a specific error and abort processing.

#### Goalkeepers

Goalkeeper attributes are listed in two columns as tuples of (string, integer) where string is the attribute and the integer is the respective value.

Column 1: Labelled "GOALKEEPING" contains the following attributes:
Aerial Reach
Command of Area
Communication
Eccentricity
First Touch
Handling
Kicking
One on Ones
Passing
Punching (Tendency)
Reflexes
Rushing Out (Tendency)
Throwing

Column 2: Labelled "MENTAL" contains the following attributes:
Aggression
Anticipation
Bravery
Composure
Concentration
Decisions
Determination
Flair
Leadership
Off The Ball
Positioning
Teamwork
Vision
Work Rate

A third column contains two sections:

Column 3, Section 1: Labelled "Physical" contains the following attributes:
Acceleration
Agility
Balance
Jumping Reach
Natural Fittness
Pace
Stamina
Strength

Column 3, Section 2: Labelled "TECHNICAL" contains the following attributes:
Free Kick Taking
Penalty Taking
Technique

#### Field Players

Field player attributes are listed in three columns as tuples of (string, integer) where string is the attribute and the integer is the respective value.

Column 1: Labelled "TECHNICAL" contains the following attributes:
Corners
Crossing
Dribbling
Finishing
First Touch
Free Kick Taking
Heading
Long Shots
Long Throws
Marking
Passing
Penalty Taking
Tackling
Technique

Column 2: Labelled "MENTAL" contains the following attributes:
Aggression
Anticipation
Bravery
Composure
Concentration
Decisions
Determination
Flair
Leadership
Off The Ball
Positioning
Teamwork
Vision
Work Rate

Column 3: Labelled "Physical" contains the following attributes:
Acceleration
Agility
Balance
Jumping Reach
Natural Fittness
Pace
Stamina
Strength

## Output

We are going to print out the extracted values in the following order and separated by tab characters:
Player name
Age
Footedness (l for left-footed, r for right-footed, lr for both)
Corners
Crossing
Dribbling
Finishing
First Touch
Free Kick Taking
Heading
Long Shots
Long Throws
Marking
Passing
Penalty Taking
Tackling
Technique
Aggression
Anticipation
Bravery
Composure
Concentration
Decisions
Determination
Flair
Leadership
Off The Ball
Positioning
Teamwork
Vision
Work Rate
Acceleration
Agility
Balance
Jumping Reach
Natural Fittness
Pace
Stamina
Strength
Aerial Reach
Command of Area
Communication
Eccentricity
Handling
Kicking
One on Ones
Punching (Tendency)
Reflexes
Rushing Out (Tendency)
Throwing
## Technical Implementation

### OCR Technology
The tool uses Tesseract OCR for local text extraction from screenshots, avoiding external service dependencies and associated costs.

### CLI Interface
The tool follows the existing fm_data CLI patterns and infrastructure:

- **Binary name**: `fm_image`
- **Config file support**: Reuses existing configuration system
- **Authentication**: Includes Google OAuth parameters for potential future Google Sheets integration
- **Logging**: Supports verbose logging options (`-v` flag)
- **Progress tracking**: Integrates with existing progress system

### Input Requirements
- **File format**: Single PNG image file
- **Input method**: Image file path passed as command line argument
- **Image content**: Football Manager player screenshot containing all required sections

### Error Handling
The tool implements strict error handling:
- **Missing attributes**: If any required attribute cannot be extracted, report specific error and abort
- **Footedness detection**: If circle colors cannot be clearly identified as green/yellow, raise error and abort
- **Missing sections**: If required sections (player name, age, footedness, attributes) are not found, abort with descriptive error
- **OCR failures**: If text extraction fails for critical elements, report error and abort

### Output Format
All players (goalkeepers and field players) use the same tab-separated output format. Missing attributes for specific player types are output as 0.