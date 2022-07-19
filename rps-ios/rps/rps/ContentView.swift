//
//  ContentView.swift
//  rps
//
//  Created by Harry on 2022/07/19.
//

import SwiftUI

enum Rps: UInt8 {
    case Rock = 0
    case Paper
    case Scissors
}

struct ContentView: View {
    @State private var board: String
    let game = create_game()
    
    init() {
        self.board = "press a button to play"
    }
    
    func refresh() {
        let board_cstr = make_score_board(game)
        self.board = String(cString: board_cstr!)
        free_score_board(UnsafeMutablePointer(mutating: board_cstr))
    }
    
    func play(choice: Rps) {
        play_game(self.game, choice.rawValue)
        refresh()
    }
    
    var body: some View {
        VStack {
            Text(board)
            Spacer()
            Button("Rock") {
                self.play(choice : Rps.Rock)
            }
            Spacer()
            Button("Paper") {
                self.play(choice : Rps.Paper)
            }
            Spacer()
            Button("Sicssors") {
                self.play(choice : Rps.Scissors)
            }
            Spacer()
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
