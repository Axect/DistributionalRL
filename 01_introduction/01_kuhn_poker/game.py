import random

class Player:
    def __init__(self, name, is_ai=True):
        self.name = name
        self.hand = None
        self.is_ai = is_ai

    def draw(self, deck):
        self.hand = deck.draw_card()

    def action(self):
        if self.is_ai:
            # a simple AI that randomly chooses an action
            return random.choice(['bet', 'pass'])
        else:
            while True:
                print(f"{self.name}, your hand is {self.hand}.")
                action = input(f"{self.name}, do you want to 'bet' or 'pass'? ").lower()
                if action in ['bet', 'pass']:
                    return action
                else:
                    print("Invalid action, please enter 'bet' or 'pass'.")

class Deck:
    def __init__(self):
        self.cards = ['Jack', 'Queen', 'King']
        random.shuffle(self.cards)

    def draw_card(self):
        return self.cards.pop()

class Game:
    def __init__(self, player1, player2):
        self.players = [player1, player2]
        self.deck = Deck()

    def play_round(self):
        for player in self.players:
            player.draw(self.deck)

        for player in self.players:
            action = player.action()
            print(f"{player.name} chooses to {action}")

            if action == 'bet':
                other_player = self.players[0] if player is self.players[1] else self.players[1]
                other_action = other_player.action()

                if other_action == 'bet':
                    print(f"{other_player.name} calls the bet.")
                    return self.determine_winner(player, other_player)
                else:
                    print(f"{other_player.name} folds.")
                    return player

        return self.determine_winner(*self.players)

    def determine_winner(self, player1, player2):
        # assuming 'Jack' < 'Queen' < 'King'
        card_values = {'Jack': 1, 'Queen': 2, 'King': 3}

        if card_values[player1.hand] > card_values[player2.hand]:
            return player1
        else:
            return player2

# create players and a game
player1 = Player("Player 1", is_ai=False)
player2 = Player("Player 2")
game = Game(player1, player2)

# play a round and print the winner
winner = game.play_round()
print(f"The winner is {winner.name}!")
